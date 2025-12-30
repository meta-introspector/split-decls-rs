// Generated macro for mismatch_param_name (function)
macro_rules! Depcrate_mismatching_type_param_ordermismatch_param_name {
() => {
// Module: crate::mismatching_type_param_order
// Provides: {"mismatch_param_name"}
// Dependencies: {}
fn mismatch_param_name (i : usize , impl_param_name : & String , type_param_names : & FxHashMap < & String , usize >) -> bool { if let Some (j) = type_param_names . get (impl_param_name) && i != * j { return true ; } false }
};
}
