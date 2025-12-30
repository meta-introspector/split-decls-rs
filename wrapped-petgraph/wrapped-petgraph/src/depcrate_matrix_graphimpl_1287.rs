// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_matrix_graphimpl_1287 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1287"}
// Dependencies: {}
impl < T : Zero > From < NotZero < T > > for Option < T > { fn from (not_zero : NotZero < T >) -> Self { if ! not_zero . is_null () { Some (not_zero . 0) } else { None } } }
};
}
