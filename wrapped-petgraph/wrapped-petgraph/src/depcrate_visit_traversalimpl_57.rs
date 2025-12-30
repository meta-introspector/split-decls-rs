// Generated macro for impl_57 (impl)
macro_rules! Depcrate_visit_traversalimpl_57 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_57"}
// Dependencies: {}
impl < N , VM > Default for Dfs < N , VM > where VM : Default , { fn default () -> Self { Dfs { stack : Vec :: new () , discovered : VM :: default () , } } }
};
}
