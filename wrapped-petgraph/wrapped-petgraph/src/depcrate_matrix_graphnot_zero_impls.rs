// Generated macro for not_zero_impls (macro)
macro_rules! Depcrate_matrix_graphnot_zero_impls {
() => {
// Module: crate::matrix_graph
// Provides: {"not_zero_impls"}
// Dependencies: {}
macro_rules ! not_zero_impls { ($ ($ t : ty) ,*) => { $ (not_zero_impl ! ($ t , 0) ;) * } }
};
}
