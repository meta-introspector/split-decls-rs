macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! opt_ret_type {
    () => {
        deps!();
        fn opt_ret_type (p : & mut Parser < '_ >) -> bool { if p . at (T ! [->]) { let m = p . start () ; p . bump (T ! [->]) ; types :: type_no_bounds (p) ; m . complete (p , RET_TYPE) ; true } else { false } }
    };
}

opt_ret_type!();