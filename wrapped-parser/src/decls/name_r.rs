macro_rules! deps {
    () => {
        TokenSet!();
        Parser!();
    };
}

macro_rules! name_r {
    () => {
        deps!();
        fn name_r (p : & mut Parser < '_ > , recovery : TokenSet) { if p . at (IDENT) { let m = p . start () ; p . bump (IDENT) ; m . complete (p , NAME) ; } else { p . err_recover ("expected a name" , recovery) ; } }
    };
}

name_r!()