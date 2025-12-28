macro_rules! deps {
    () => {
        Parser!();
        TokenSet!();
    };
}

macro_rules! name {
    () => {
        deps!();
        fn name (p : & mut Parser < '_ >) { name_r (p , TokenSet :: EMPTY) ; }
    };
}

name!()