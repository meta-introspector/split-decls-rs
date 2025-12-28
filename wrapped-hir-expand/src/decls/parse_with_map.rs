macro_rules! deps {
    () => {
        SpanMap!();
        ExpandDatabase!();
        ExpansionSpanMap!();
        HirFileId!();
    };
}

macro_rules! parse_with_map {
    () => {
        deps!();
        pub (crate) fn parse_with_map (db : & dyn ExpandDatabase , file_id : HirFileId ,) -> (Parse < SyntaxNode > , SpanMap) { match file_id { HirFileId :: FileId (file_id) => { (db . parse (file_id) . to_syntax () , SpanMap :: RealSpanMap (db . real_span_map (file_id))) } HirFileId :: MacroFile (macro_file) => { let (parse , map) = db . parse_macro_expansion (macro_file) . value ; (parse , SpanMap :: ExpansionSpanMap (map)) } } }
    };
}

parse_with_map!();