// Generated macro for RtlIsRoot (function)
macro_rules! Depcrate_ntrtlRtlIsRoot {
() => {
// Module: crate::ntrtl
// Provides: {"RtlIsRoot"}
// Dependencies: {}
# [inline] pub unsafe fn RtlIsRoot (Links : * const RTL_SPLAY_LINKS) -> bool { (* Links) . Parent as * const _ == Links }
};
}
