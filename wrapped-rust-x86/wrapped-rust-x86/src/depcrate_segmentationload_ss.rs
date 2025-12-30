// Generated macro for load_ss (function)
macro_rules! Depcrate_segmentationload_ss {
() => {
// Module: crate::segmentation
// Provides: {"load_ss"}
// Dependencies: {}
# [doc = " Reload stack segment register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn load_ss (sel : SegmentSelector) { asm ! ("movw {0:x}, %ss" , in (reg) sel . bits () , options (att_syntax)) ; }
};
}
