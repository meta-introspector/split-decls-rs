// Generated macro for RtlIsLeftChild (function)
macro_rules! Depcrate_ntrtlRtlIsLeftChild {
() => {
// Module: crate::ntrtl
// Provides: {"RtlIsLeftChild"}
// Dependencies: {}
# [inline] pub unsafe fn RtlIsLeftChild (Links : * const RTL_SPLAY_LINKS) -> bool { RtlLeftChild (& * RtlParent (& * Links)) as * const _ == Links }
};
}
