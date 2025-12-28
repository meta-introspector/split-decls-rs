macro_rules! ty_pat {
    () => {
        fn ty_pat (kind : TyPatKind , span : Span) -> Box < TyPat > { Box :: new (TyPat { id : DUMMY_NODE_ID , kind , span , tokens : None }) }
    };
}

ty_pat!()