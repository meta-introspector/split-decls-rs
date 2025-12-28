macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! name_ref_or_index {
    () => {
        deps!();
        fn name_ref_or_index (p : & mut Parser < '_ >) { assert ! (p . at (IDENT) || p . at (INT_NUMBER)) ; let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; }
    };
}

name_ref_or_index!();