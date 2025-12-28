macro_rules! escape_string {
    () => {
        fn escape_string (s : & [u8]) -> String { match str :: from_utf8 (s) { Ok (s) => s . to_owned () , Err (_) => format ! ("Non-UTF-8 output: {}" , s . escape_ascii ()) , } }
    };
}

escape_string!();