macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! opt_rename {
    () => {
        deps!();
        fn opt_rename (p : & mut Parser < '_ >) { if p . at (T ! [as]) { let m = p . start () ; p . bump (T ! [as]) ; if ! p . eat (T ! [_]) { name (p) ; } m . complete (p , RENAME) ; } }
    };
}

opt_rename!()