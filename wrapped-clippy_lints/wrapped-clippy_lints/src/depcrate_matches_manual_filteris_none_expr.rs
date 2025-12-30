// Generated macro for is_none_expr (function)
macro_rules! Depcrate_matches_manual_filteris_none_expr {
() => {
// Module: crate::matches::manual_filter
// Provides: {"is_none_expr"}
// Dependencies: {}
fn is_none_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Some (inner_expr) = peels_blocks_incl_unsafe_opt (expr) { return inner_expr . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionNone) ; } false }
};
}
