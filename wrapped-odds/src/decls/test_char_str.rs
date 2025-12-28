macro_rules! deps {
    () => {
        CharStr!();
    };
}

macro_rules! test_char_str {
    () => {
        deps!();
        # [test] fn test_char_str () { let s = CharStr :: new ('α') ; assert_eq ! (& s [..] , "α") ; }
    };
}

test_char_str!()