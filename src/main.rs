fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_gt_100(n: i32) {
    if n > 200 {
        println!("Huge Number");
    } else if n > 99 {
        println!("Big Number");
    } else {
        println!("Small Number");
    }
}

fn test_loop() {
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }
        println!("{a}");
        a += 1;
    }
    while a < 10 {
        println!("{a}");
        a += 1;
    }
}

fn tuples_() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let sum_ = add(10, 20);
    is_gt_100(205);
    test_loop();

    println!("{:?}", sum_);
    println!("{sum_:?}"); // Debug Version
    println!("{sum_}"); // User Version

    let some_int: i32 = 2;
    match some_int {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("other"),
    }

    enum Direction {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }
    let direction = Direction::UP;
    match direction {
        Direction::UP => println!("UP"),
        Direction::DOWN => println!("DOWN"),
        Direction::LEFT => println!("LEFT"),
        Direction::RIGHT => println!("RIGHT"),
    }

    struct Rectangle {
        direction: Direction,
        length: f64,
        breadth: f64,
    }

    let bed = Rectangle {
        direction: Direction::DOWN,
        length: 10.0,
        breadth: 20.0,
    };

    println!("length = {:?} , breadth = {:?}", bed.length, bed.breadth);

    let tuple_data = tuples_();
    let (x, y, z) = tuples_();
    println!("{x} {:?}", tuple_data.0);
    println!("{y} {:?}", tuple_data.1);
    println!("{z} {:?}", tuple_data.2);

    let my_num = 10 < 4;
    println!("{my_num}");

    enum Light {
        Bright,
        Dull,
    }

    let light = Light::Bright;

    fn display_light(light: &Light) {
        match light {
            Light::Bright => println!("Bright"),
            Light::Dull => println!("Bright"),
        }
    }
    // Borrowing if i dont borrow => light is in scope of main of main owns it once it is passed to display_light the display_light owns it
    // once the function is executed display light will delete the light variable as rust follow ownership & borrow model so here we have borrowed the light from main
    display_light(&light);
    display_light(&light);

    struct Temperature {
        degrees_f: f64,
    }

    impl Temperature {
        fn freezing() -> Self {
            Self { degrees_f: 30.0 }
        }

        fn show_temp(&self) {
            println!("{:?} is the current temperature.", self.degrees_f)
        }

        fn boiling() -> Self {
            Self { degrees_f: 100.0 }
        }
    }

    let hot = Temperature { degrees_f: 50.2 };

    hot.show_temp();

    Temperature::boiling();
    Temperature::freezing();

    // List / Vectors

    let my_list = vec![1, 2, 3];
    println!("{:?} first item", my_list[0]);

    let mut mutable_list: Vec<i32> = Vec::new();
    mutable_list.push(10);
    mutable_list.push(11);
    mutable_list.push(10);
    mutable_list.pop();
    mutable_list.len();

    for num in &mutable_list {
        println!("{:?}", num)
    }

    let string_ = "by default it is borrowed";

    fn print(to_print: &str) {
        println!("{:?}", to_print);
    }

    print(string_);

    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: i32,
    }

    // we cannot use string_ to create the Person object as struct requires owned string and not borrowed string,
    //  as it will not be able to clean memmory after returing the struct obj

    // ways to create owned String.

    let person_obj = vec![
        Person {
            name: "Vishal".to_owned(),
            age: 23,
        },
        Person {
            name: String::from("Alex"),
            age: 20,
        },
    ];

    for person in person_obj {
        print(&person.name);
        println!("{}", person.age);
        // As we used Derive Debug we can print it directly note :- if you are using enum inside struct make sure to derive it too
        println!("{:?}", person);
    }

    // Advanced match

    enum Discount {
        Percent(i32),
        Flat(i32),
    }

    struct Ticket {
        event: String,
        price: i32,
    }

    let discount_ = Discount::Flat(12);
    match discount_ {
        Discount::Flat(2) => println!("discount of 2% flat"),
        Discount::Flat(amount) => println!("discount of {}%", amount),
        _ => (),
    }

    let concert_ = Ticket {
        event: "Music Concert".to_owned(),
        price: 50,
    };

    match concert_ {
        Ticket { price: 50, event } => println!("{} at price 50.0", event),
        Ticket { price, event } => println!("{} at price {}", event, price),
    }
}
