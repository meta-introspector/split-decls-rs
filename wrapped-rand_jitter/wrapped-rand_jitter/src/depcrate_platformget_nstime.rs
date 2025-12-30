// Generated macro for get_nstime (function)
macro_rules! Depcrate_platformget_nstime {
() => {
// Module: crate::platform
// Provides: {"get_nstime"}
// Dependencies: {}
# [cfg (target_os = "windows")] pub fn get_nstime () -> u64 { use winapi ; unsafe { let mut t = super :: mem :: zeroed () ; winapi :: um :: profileapi :: QueryPerformanceCounter (& mut t) ; * t . QuadPart () as u64 } }
};
}
