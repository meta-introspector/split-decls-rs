macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! name_ref_mod_path_or_index {
    () => {
        deps!();
        fn name_ref_mod_path_or_index (p : & mut Parser < '_ >) { if p . at_ts (PATH_NAME_REF_OR_INDEX_KINDS) { let m = p . start () ; p . bump_any () ; m . complete (p , NAME_REF) ; } else { p . err_and_bump ("expected integer, identifier, `self`, `super`, `crate`, or `Self`") ; } }
    };
}

name_ref_mod_path_or_index!()