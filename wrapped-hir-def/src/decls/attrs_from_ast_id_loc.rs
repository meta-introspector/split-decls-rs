macro_rules! deps {
    () => {
        DefDatabase!();
        HasModule!();
        Attrs!();
        AstIdLoc!();
    };
}

macro_rules! attrs_from_ast_id_loc {
    () => {
        deps!();
        fn attrs_from_ast_id_loc < 'db , N : AstIdNode + HasAttrs > (db : & (dyn DefDatabase + 'db) , lookup : impl Lookup < Database = dyn DefDatabase , Data = impl AstIdLoc < Ast = N > + HasModule > ,) -> Attrs { let loc = lookup . lookup (db) ; let source = loc . source (db) ; let span_map = db . span_map (source . file_id) ; let cfg_options = loc . krate (db) . cfg_options (db) ; Attrs (RawAttrs :: new_expanded (db , & source . value , span_map . as_ref () , cfg_options)) }
    };
}

attrs_from_ast_id_loc!()