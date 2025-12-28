macro_rules! deps {
    () => {
        Cursor!();
        FrontmatterAllowed!();
        RawStrError!();
    };
}

macro_rules! check_raw_str {
    () => {
        deps!();
        fn check_raw_str (s : & str , expected : Result < u8 , RawStrError >) { let s = & format ! ("r{}" , s) ; let mut cursor = Cursor :: new (s , FrontmatterAllowed :: No) ; cursor . bump () ; let res = cursor . raw_double_quoted_string (0) ; assert_eq ! (res , expected) ; }
    };
}

check_raw_str!()