// Generated macro for not_zero_impl (macro)
macro_rules! Depcrate_matrix_graphnot_zero_impl {
() => {
// Module: crate::matrix_graph
// Provides: {"not_zero_impl"}
// Dependencies: {}
macro_rules ! not_zero_impl { ($ t : ty ,$ z : expr) => { impl Zero for $ t { fn zero () -> Self { $ z as $ t } # [allow (clippy :: float_cmp)] fn is_zero (& self) -> bool { self == & Self :: zero () } } } ; }
};
}
