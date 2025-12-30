// Generated macro for impl_item_search_pat (function)
macro_rules! Depcrate_check_proc_macroimpl_item_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"impl_item_search_pat"}
// Dependencies: {}
fn impl_item_search_pat (item : & ImplItem < '_ >) -> (Pat , Pat) { let (mut start_pat , end_pat) = match & item . kind { ImplItemKind :: Const (..) => (Pat :: Str ("const") , Pat :: Str (";")) , ImplItemKind :: Type (..) => (Pat :: Str ("type") , Pat :: Str (";")) , ImplItemKind :: Fn (sig , ..) => (fn_header_search_pat (sig . header) , Pat :: Str ("")) , } ; if let ImplItemImplKind :: Inherent { vis_span , .. } = item . impl_kind && ! vis_span . is_empty () { start_pat = Pat :: Str ("pub") ; } (start_pat , end_pat) }
};
}
