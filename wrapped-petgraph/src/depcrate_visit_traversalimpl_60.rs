// Generated macro for impl_60 (impl)
macro_rules! Depcrate_visit_traversalimpl_60 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_60"}
// Dependencies: {}
impl < N , VM > Default for DfsPostOrder < N , VM > where VM : Default , { fn default () -> Self { DfsPostOrder { stack : Vec :: new () , discovered : VM :: default () , finished : VM :: default () , } } }
};
}
