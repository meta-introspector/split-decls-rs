macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! opt_visibility {
    () => {
        deps!();
        fn opt_visibility (p : & mut Parser < '_ > , in_tuple_field : bool) -> bool { if ! p . at (T ! [pub]) { return false ; } let m = p . start () ; p . bump (T ! [pub]) ; if p . at (T ! ['(']) { match p . nth (1) { T ! [crate] | T ! [self] | T ! [super] | T ! [ident] | T ! [')'] if p . nth (2) != T ! [:] => { if ! (in_tuple_field && matches ! (p . nth (1) , T ! [ident] | T ! [')'])) { p . bump (T ! ['(']) ; paths :: vis_path (p) ; p . expect (T ! [')']) ; } } T ! [in] => { p . bump (T ! ['(']) ; p . bump (T ! [in]) ; paths :: vis_path (p) ; p . expect (T ! [')']) ; } _ => { } } } m . complete (p , VISIBILITY) ; true }
    };
}

opt_visibility!();