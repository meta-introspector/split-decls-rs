macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! name_ref_or_upper_self {
    () => {
        deps!();
        fn name_ref_or_upper_self (p : & mut Parser < '_ >) { if matches ! (p . current () , T ! [ident] | T ! [Self]) { let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; } else { p . err_and_bump ("expected identifier or `Self`") ; } }
    };
}

name_ref_or_upper_self!();