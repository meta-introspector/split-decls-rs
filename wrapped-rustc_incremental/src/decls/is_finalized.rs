macro_rules! is_finalized {
    () => {
        fn is_finalized (directory_name : & str) -> bool { ! directory_name . ends_with ("-working") }
    };
}

is_finalized!()