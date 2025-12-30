// Generated macro for is_valid_item_kind (function)
macro_rules! Depcrate_single_call_fnis_valid_item_kind {
() => {
// Module: crate::single_call_fn
// Provides: {"is_valid_item_kind"}
// Dependencies: {}
# [doc = " Whether a called function is a kind of item that the lint cares about."] # [doc = " For example, calling an `extern \"C\" { fn fun(); }` only once is totally fine and does not"] # [doc = " to be considered."] fn is_valid_item_kind (cx : & LateContext < '_ > , def_id : LocalDefId) -> bool { matches ! (cx . tcx . hir_node_by_def_id (def_id) , Node :: Item (_) | Node :: ImplItem (_) | Node :: TraitItem (_)) }
};
}
