// Generated macro for DebuggerScripts (struct)
macro_rules! Depcrate_core_build_steps_distDebuggerScripts {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"DebuggerScripts"}
// Dependencies: {}
# [doc = " Copies debugger scripts for `target` into the given compiler `sysroot`."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct DebuggerScripts { # [doc = " Sysroot of a compiler into which will the debugger scripts be copied to."] pub sysroot : PathBuf , pub target : TargetSelection , }
};
}
