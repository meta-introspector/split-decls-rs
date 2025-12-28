macro_rules! deps {
    () => {
        Id!();
        Node!();
    };
}

macro_rules! id_name {
    () => {
        deps!();
        fn id_name < 'a > (n : & Node) -> Id < 'a > { Id :: new (format ! ("N{}" , * n)) . unwrap () }
    };
}

id_name!();