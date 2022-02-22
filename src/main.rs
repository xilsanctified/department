use std::{
    collections::HashMap,
    io::{self, Write},
};

struct Company {
    department: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            department: HashMap::new(),
        }
    }

    fn add_employee(&mut self, department: &str, first_name: &str, last_name: &str) {
        let name: Vec<String> = Vec::new();

        let name_list = self
            .department
            .entry(department.to_string())
            .or_insert(name);
        name_list.push(format!("{} {}", first_name, last_name));
    }

    fn new_employee(&mut self) {
        menu_title("New Employee");
        loop {
            println!("Syntax: (add | remove) [first name] [last name] (to | from) [department]");
            let buffer: String = user_input().to_lowercase();
            let command: Vec<&str> = buffer.split_whitespace().collect();

            if command.len() != 5 {
                println!("\nInvalid command. Try again!\n");
                continue;
            }

            let operation = (*command.get(0).unwrap(), *command.get(3).unwrap());
            match operation {
                ("add", "to") => {
                    self.add_employee(
                        *command.get(4).unwrap(),
                        *command.get(1).unwrap(),
                        *command.get(2).unwrap(),
                    );
                    break;
                }
                ("remove", "from") => {
                    println!(
                        "{}",
                        format!(
                            "You must not remove {} {}!",
                            *command.get(1).unwrap(),
                            *command.get(2).unwrap()
                        )
                    );
                    break;
                }
                _ => {
                    println!("\nInput error, Try again!\n");
                    continue;
                }
            }
        }

        key_press();
    }

    fn list_departments(&self) {
        menu_title("Department List");
        for each in &self.department {
            println!("{}", each.0);
        }
        key_press();
    }

    fn list_employees(&self) {
        menu_title("Employee List");
        for each in &self.department {
            println!("{:?}", each.1);
        }
        key_press();
    }
}

fn menu_title(title: &str) {
    println!("-> {}\n", title);
}

fn user_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed.");
    buffer
}

fn key_press() {
    print!("Press [enter] to continue...");
    let flush = io::stdout().flush();
    let mut _buffer = user_input();
    match flush {
        Ok(()) => (),
        _ => panic!("std::io Error!"),
    }
}

fn main() -> io::Result<()> {
    // Department -> Vec<employees>
    let mut company = Company::new();

    loop {
        menu_title("Company Directory");
        println!("\n'n' : New Entry\n'l' : List Departments\n'e' : List Employees\n'q' : Quit");
        print!("\nEnter your choice: ");

        io::stdout().flush()?;
        let user_input = user_input();

        match user_input.trim() {
            "n" => company.new_employee(),
            "l" => company.list_departments(),
            "e" => company.list_employees(),
            "q" => return Ok(()),
            _ => continue,
        }
    }
}
