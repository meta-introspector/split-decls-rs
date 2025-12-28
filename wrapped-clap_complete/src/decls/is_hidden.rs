macro_rules! is_hidden {
    () => {
        fn is_hidden (file_name : & OsStr) -> bool { file_name . starts_with (".") }
    };
}

is_hidden!();