macro_rules! deps {
    () => {
        DefDatabase!();
        HygieneId!();
    };
}

macro_rules! handle_macro_def_scope {
    () => {
        deps!();
        # [inline] fn handle_macro_def_scope (db : & dyn DefDatabase , hygiene_id : & mut HygieneId , hygiene_info : & mut Option < (SyntaxContext , MacroDefId) > , macro_id : & MacroDefId ,) { if let Some ((parent_ctx , label_macro_id)) = hygiene_info && label_macro_id == macro_id { * hygiene_id = HygieneId :: new (parent_ctx . opaque_and_semitransparent (db)) ; * hygiene_info = parent_ctx . outer_expn (db) . map (| expansion | { let expansion = db . lookup_intern_macro_call (expansion . into ()) ; (parent_ctx . parent (db) , expansion . def) }) ; } }
    };
}

handle_macro_def_scope!()