macro_rules! split_nonutf8_once {
    () => {
        fn split_nonutf8_once (b : & OsStr) -> (& str , Option < & OsStr >) { match b . try_str () { Ok (s) => (s , None) , Err (err) => { let (valid , after_valid) = unsafe { ext :: split_at (b , err . valid_up_to ()) } ; let valid = valid . try_str () . unwrap () ; (valid , Some (after_valid)) } } }
    };
}

split_nonutf8_once!()