macro_rules! trim_trailing_spaces {
    () => {
        fn trim_trailing_spaces (doc : & mut String) { doc . truncate (doc . trim_end_matches (' ') . len ()) ; }
    };
}

trim_trailing_spaces!();