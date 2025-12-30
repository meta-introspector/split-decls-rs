// Generated macro for load_fs (function)
macro_rules! Depcrate_segmentationload_fs {
() => {
// Module: crate::segmentation
// Provides: {"load_fs"}
// Dependencies: {}
# [doc = " Reload fs segment register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn load_fs (sel : SegmentSelector) { asm ! ("movw {0:x}, %fs" , in (reg) sel . bits () , options (att_syntax)) ; }
};
}
