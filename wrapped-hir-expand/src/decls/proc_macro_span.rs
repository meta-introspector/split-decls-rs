macro_rules! deps {
    () => {
        ExpandDatabase!();
        AstId!();
    };
}

macro_rules! proc_macro_span {
    () => {
        deps!();
        fn proc_macro_span (db : & dyn ExpandDatabase , ast : AstId < ast :: Fn >) -> Span { let root = db . parse_or_expand (ast . file_id) ; let ast_id_map = & db . ast_id_map (ast . file_id) ; let span_map = & db . span_map (ast . file_id) ; let node = ast_id_map . get (ast . value) . to_node (& root) ; let range = ast :: HasName :: name (& node) . map_or_else (| | node . syntax () . text_range () , | name | name . syntax () . text_range ()) ; span_map . span_for_range (range) }
    };
}

proc_macro_span!()