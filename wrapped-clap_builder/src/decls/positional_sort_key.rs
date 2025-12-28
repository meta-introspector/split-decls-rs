macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! positional_sort_key {
    () => {
        deps!();
        fn positional_sort_key (arg : & Arg) -> (usize , String) { (arg . get_index () . unwrap_or (0) , String :: new ()) }
    };
}

positional_sort_key!()