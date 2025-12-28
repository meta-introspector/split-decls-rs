macro_rules! deps {
    () => {
        HirFileId!();
        ExpandDatabase!();
    };
}

macro_rules! syntax_context {
    () => {
        deps!();
        fn syntax_context (db : & dyn ExpandDatabase , file : HirFileId , edition : Edition) -> SyntaxContext { match file { HirFileId :: FileId (_) => SyntaxContext :: root (edition) , HirFileId :: MacroFile (m) => { let kind = db . lookup_intern_macro_call (m) . kind ; db . macro_arg_considering_derives (m , & kind) . 2 . ctx } } }
    };
}

syntax_context!()