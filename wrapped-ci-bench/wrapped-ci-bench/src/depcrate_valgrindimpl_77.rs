// Generated macro for impl_77 (impl)
macro_rules! Depcrate_valgrindimpl_77 {
() => {
// Module: crate::valgrind
// Provides: {"impl_77"}
// Dependencies: {}
impl Drop for CountInstructions { fn drop (& mut self) { # [cfg (target_os = "linux")] crabgrind :: callgrind :: toggle_collect () ; } }
};
}
