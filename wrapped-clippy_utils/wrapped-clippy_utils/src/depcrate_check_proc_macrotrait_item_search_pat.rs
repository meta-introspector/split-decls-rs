// Generated macro for trait_item_search_pat (function)
macro_rules! Depcrate_check_proc_macrotrait_item_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"trait_item_search_pat"}
// Dependencies: {}
fn trait_item_search_pat (item : & TraitItem < '_ >) -> (Pat , Pat) { match & item . kind { TraitItemKind :: Const (..) => (Pat :: Str ("const") , Pat :: Str (";")) , TraitItemKind :: Type (..) => (Pat :: Str ("type") , Pat :: Str (";")) , TraitItemKind :: Fn (sig , ..) => (fn_header_search_pat (sig . header) , Pat :: Str ("")) , } }
};
}
