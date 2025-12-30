// Generated macro for is_android_gdb_target (function)
macro_rules! Depcrate_debuggersis_android_gdb_target {
() => {
// Module: crate::debuggers
// Provides: {"is_android_gdb_target"}
// Dependencies: {}
# [doc = " Returns `true` if the given target is an Android target for the"] # [doc = " purposes of GDB testing."] pub (crate) fn is_android_gdb_target (target : & str) -> bool { matches ! (& target [..] , "arm-linux-androideabi" | "armv7-linux-androideabi" | "aarch64-linux-android") }
};
}
