// Generated macro for impl_206 (impl)
macro_rules! Depcrate_astimpl_206 {
() => {
// Module: crate::ast
// Provides: {"impl_206"}
// Dependencies: {}
impl < N : AstNode > Iterator for AstChildren < N > { type Item = N ; fn next (& mut self) -> Option < N > { self . inner . find_map (N :: cast) } }
};
}
