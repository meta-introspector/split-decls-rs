macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! name_ref_mod_path {
    () => {
        deps!();
        fn name_ref_mod_path (p : & mut Parser < '_ >) { if p . at_ts (PATH_NAME_REF_KINDS) { let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; } else { p . err_and_bump ("expected identifier, `self`, `super`, `crate`, or `Self`") ; } }
    };
}

name_ref_mod_path!()