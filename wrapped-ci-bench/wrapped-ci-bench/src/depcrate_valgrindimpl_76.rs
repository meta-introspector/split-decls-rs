// Generated macro for impl_76 (impl)
macro_rules! Depcrate_valgrindimpl_76 {
() => {
// Module: crate::valgrind
// Provides: {"impl_76"}
// Dependencies: {}
impl CountInstructions { pub (crate) fn start () -> Self { # [cfg (target_os = "linux")] crabgrind :: callgrind :: toggle_collect () ; Self } }
};
}
