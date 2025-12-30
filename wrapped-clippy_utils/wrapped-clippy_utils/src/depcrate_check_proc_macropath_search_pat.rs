// Generated macro for path_search_pat (function)
macro_rules! Depcrate_check_proc_macropath_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"path_search_pat"}
// Dependencies: {}
fn path_search_pat (path : & Path < '_ >) -> (Pat , Pat) { let (head , tail) = match path . segments { [] => return (Pat :: Str ("") , Pat :: Str ("")) , [p] => (Pat :: Sym (p . ident . name) , p) , [.. , tail] => (Pat :: Str ("") , tail) , } ; (head , if tail . args . is_some () { Pat :: Str (">") } else { Pat :: Sym (tail . ident . name) } ,) }
};
}
