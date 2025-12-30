// Generated macro for hygiene_info (function)
macro_rules! Depcrate_resolverhygiene_info {
() => {
// Module: crate::resolver
// Provides: {"hygiene_info"}
// Dependencies: {}
# [inline] fn hygiene_info (db : & dyn DefDatabase , hygiene_id : HygieneId ,) -> Option < (SyntaxContext , MacroDefId) > { if ! hygiene_id . is_root () { let ctx = hygiene_id . lookup () ; ctx . outer_expn (db) . map (| expansion | { let expansion = db . lookup_intern_macro_call (expansion . into ()) ; (ctx . parent (db) , expansion . def) }) } else { None } }
};
}
