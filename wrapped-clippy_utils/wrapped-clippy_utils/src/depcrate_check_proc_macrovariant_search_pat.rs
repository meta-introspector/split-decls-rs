// Generated macro for variant_search_pat (function)
macro_rules! Depcrate_check_proc_macrovariant_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"variant_search_pat"}
// Dependencies: {}
fn variant_search_pat (v : & Variant < '_ >) -> (Pat , Pat) { match v . data { VariantData :: Struct { .. } => (Pat :: Sym (v . ident . name) , Pat :: Str ("}")) , VariantData :: Tuple (..) => (Pat :: Sym (v . ident . name) , Pat :: Str ("")) , VariantData :: Unit (..) => (Pat :: Sym (v . ident . name) , Pat :: Sym (v . ident . name)) , } }
};
}
