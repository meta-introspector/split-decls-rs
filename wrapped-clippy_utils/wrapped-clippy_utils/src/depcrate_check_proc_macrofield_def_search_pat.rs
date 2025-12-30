// Generated macro for field_def_search_pat (function)
macro_rules! Depcrate_check_proc_macrofield_def_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"field_def_search_pat"}
// Dependencies: {}
fn field_def_search_pat (def : & FieldDef < '_ >) -> (Pat , Pat) { if def . vis_span . is_empty () { if def . is_positional () { (Pat :: Str ("") , Pat :: Str ("")) } else { (Pat :: Sym (def . ident . name) , Pat :: Str ("")) } } else { (Pat :: Str ("pub") , Pat :: Str ("")) } }
};
}
