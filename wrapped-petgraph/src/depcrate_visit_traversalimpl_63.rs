// Generated macro for impl_63 (impl)
macro_rules! Depcrate_visit_traversalimpl_63 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_63"}
// Dependencies: {}
impl < N , VM > Default for Bfs < N , VM > where VM : Default , { fn default () -> Self { Bfs { stack : VecDeque :: new () , discovered : VM :: default () , } } }
};
}
