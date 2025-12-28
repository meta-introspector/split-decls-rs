macro_rules! deps {
    () => {
        ExpandDatabase!();
        HirFileId!();
    };
}

macro_rules! parse_or_expand {
    () => {
        deps!();
        # [doc = " Main public API -- parses a hir file, not caring whether it's a real"] # [doc = " file or a macro expansion."] fn parse_or_expand (db : & dyn ExpandDatabase , file_id : HirFileId) -> SyntaxNode { match file_id { HirFileId :: FileId (file_id) => db . parse (file_id) . syntax_node () , HirFileId :: MacroFile (macro_file) => { db . parse_macro_expansion (macro_file) . value . 0 . syntax_node () } } }
    };
}

parse_or_expand!();