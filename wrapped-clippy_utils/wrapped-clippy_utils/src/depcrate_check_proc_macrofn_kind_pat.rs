// Generated macro for fn_kind_pat (function)
macro_rules! Depcrate_check_proc_macrofn_kind_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"fn_kind_pat"}
// Dependencies: {}
fn fn_kind_pat (tcx : TyCtxt < '_ > , kind : & FnKind < '_ > , body : & Body < '_ > , hir_id : HirId) -> (Pat , Pat) { let (mut start_pat , end_pat) = match kind { FnKind :: ItemFn (.. , header) => (fn_header_search_pat (* header) , Pat :: Str ("")) , FnKind :: Method (.. , sig) => (fn_header_search_pat (sig . header) , Pat :: Str ("")) , FnKind :: Closure => return (Pat :: Str ("") , expr_search_pat (tcx , body . value) . 1) , } ; match tcx . hir_node (hir_id) { Node :: Item (Item { vis_span , .. }) | Node :: ImplItem (ImplItem { impl_kind : ImplItemImplKind :: Inherent { vis_span , .. } , .. }) => { if ! vis_span . is_empty () { start_pat = Pat :: Str ("pub") ; } } , Node :: ImplItem (_) | Node :: TraitItem (_) => { } , _ => start_pat = Pat :: Str ("") , } (start_pat , end_pat) }
};
}
