macro_rules! deps {
    () => {
        HygieneId!();
        DefDatabase!();
    };
}

macro_rules! hygiene_info {
    () => {
        deps!();
        # [inline] fn hygiene_info (db : & dyn DefDatabase , hygiene_id : HygieneId ,) -> Option < (SyntaxContext , MacroDefId) > { if ! hygiene_id . is_root () { let ctx = hygiene_id . lookup () ; ctx . outer_expn (db) . map (| expansion | { let expansion = db . lookup_intern_macro_call (expansion . into ()) ; (ctx . parent (db) , expansion . def) }) } else { None } }
    };
}

hygiene_info!();