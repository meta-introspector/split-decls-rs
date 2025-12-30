// Generated macro for RtlIsRightChild (function)
macro_rules! Depcrate_ntrtlRtlIsRightChild {
() => {
// Module: crate::ntrtl
// Provides: {"RtlIsRightChild"}
// Dependencies: {}
# [inline] pub unsafe fn RtlIsRightChild (Links : * const RTL_SPLAY_LINKS) -> bool { RtlRightChild (& * RtlParent (& * Links)) as * const _ == Links }
};
}
