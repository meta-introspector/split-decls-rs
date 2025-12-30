// Generated macro for is_slice_from_ref (function)
macro_rules! Depcrate_manual_option_as_sliceis_slice_from_ref {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"is_slice_from_ref"}
// Dependencies: {}
fn is_slice_from_ref (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { expr . basic_res () . is_diag_item (cx , sym :: slice_from_ref) }
};
}
