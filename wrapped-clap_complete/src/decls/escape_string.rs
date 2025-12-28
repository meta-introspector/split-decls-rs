macro_rules! escape_string {
    () => {
        fn escape_string (string : & str) -> String { string . replace ('\'' , "''") . replace ('’' , "'’") }
    };
}

escape_string!();