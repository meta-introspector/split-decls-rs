macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! lifetime {
    () => {
        deps!();
        fn lifetime (p : & mut Parser < '_ >) { assert ! (p . at (LIFETIME_IDENT)) ; let m = p . start () ; p . bump (LIFETIME_IDENT) ; m . complete (p , LIFETIME) ; }
    };
}

lifetime!()