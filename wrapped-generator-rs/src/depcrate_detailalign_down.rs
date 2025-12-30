// Generated macro for align_down (function)
macro_rules! Depcrate_detailalign_down {
() => {
// Module: crate::detail
// Provides: {"align_down"}
// Dependencies: {}
# [inline] fn align_down (sp : * mut usize) -> * mut usize { let sp = (sp as usize) & ! (16 - 1) ; sp as * mut usize }
};
}
