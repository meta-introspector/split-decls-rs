macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! name_ref_or_self {
    () => {
        deps!();
        fn name_ref_or_self (p : & mut Parser < '_ >) { if matches ! (p . current () , T ! [ident] | T ! [self]) { let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; } else { p . err_and_bump ("expected identifier or `self`") ; } }
    };
}

name_ref_or_self!();