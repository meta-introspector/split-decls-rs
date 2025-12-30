// Generated macro for impl_795 (impl)
macro_rules! Depcrate_core_build_steps_toolstateimpl_795 {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"impl_795"}
// Dependencies: {}
impl RepoState { fn state (& self) -> ToolState { if cfg ! (target_os = "linux") { self . linux } else if cfg ! (windows) { self . windows } else { unimplemented ! () } } }
};
}
