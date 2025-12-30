// Generated macro for get_arg_snippet (function)
macro_rules! Depcrate_non_zero_suggestionsget_arg_snippet {
() => {
// Module: crate::non_zero_suggestions
// Provides: {"get_arg_snippet"}
// Dependencies: {}
fn get_arg_snippet (cx : & LateContext < '_ > , arg : & Expr < '_ > , rcv_path : & rustc_hir :: PathSegment < '_ >) -> String { let arg_snippet = snippet (cx , arg . span , "..") ; if let Some (index) = arg_snippet . rfind (& format ! (".{}" , rcv_path . ident . name)) { arg_snippet [.. index] . trim () . to_string () } else { arg_snippet . to_string () } }
};
}
