macro_rules! deps {
    () => {
        InStringCtx!();
        SyntaxTreeCtx!();
    };
}

macro_rules! parse_rust_string {
    () => {
        deps!();
        fn parse_rust_string (token : SyntaxToken , ctx : & SyntaxTreeCtx) -> Option < String > { let string_node = ast :: String :: cast (token) ? ; let text = string_node . value () . ok () ? ; let mut trim_result = String :: new () ; let mut marker_positions = Vec :: new () ; let mut skipped = 0 ; let mut last_end = 0 ; for (start , part) in text . match_indices ("$0") { marker_positions . push ((start - skipped) as u32) ; trim_result . push_str (& text [last_end .. start]) ; skipped += part . len () ; last_end = start + part . len () ; } trim_result . push_str (& text [last_end .. text . len ()]) ; let parsed = SourceFile :: parse (& trim_result , span :: Edition :: CURRENT) ; if ! parsed . errors () . is_empty () { return None ; } let node : & SyntaxNode = & parsed . syntax_node () ; if node . children () . count () == 0 { return None ; } let ctx = SyntaxTreeCtx { line_index : ctx . line_index . clone () , in_string : Some (InStringCtx { offset : string_node . text_range_between_quotes () ? . start () . into () , marker_positions , }) , } ; Some (syntax_node_to_json (node , & ctx)) }
    };
}

parse_rust_string!()