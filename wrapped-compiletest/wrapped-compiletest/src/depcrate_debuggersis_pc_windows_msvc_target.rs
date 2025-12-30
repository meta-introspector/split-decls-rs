// Generated macro for is_pc_windows_msvc_target (function)
macro_rules! Depcrate_debuggersis_pc_windows_msvc_target {
() => {
// Module: crate::debuggers
// Provides: {"is_pc_windows_msvc_target"}
// Dependencies: {}
# [doc = " Returns `true` if the given target is a MSVC target for the purposes of CDB testing."] fn is_pc_windows_msvc_target (target : & str) -> bool { target . ends_with ("-pc-windows-msvc") }
};
}
