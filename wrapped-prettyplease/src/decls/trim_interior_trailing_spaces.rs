macro_rules! trim_interior_trailing_spaces {
    () => {
        fn trim_interior_trailing_spaces (doc : & mut String) { if ! doc . contains (" \n") { return ; } let mut trimmed = String :: with_capacity (doc . len ()) ; let mut lines = doc . split ('\n') . peekable () ; while let Some (line) = lines . next () { if lines . peek () . is_some () { trimmed . push_str (line . trim_end_matches (' ')) ; trimmed . push ('\n') ; } else { trimmed . push_str (line) ; } } * doc = trimmed ; }
    };
}

trim_interior_trailing_spaces!();