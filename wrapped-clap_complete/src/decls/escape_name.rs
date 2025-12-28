macro_rules! escape_name {
    () => {
        fn escape_name (name : & str) -> String { name . replace ('-' , "_") }
    };
}

escape_name!()