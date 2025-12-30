// Generated macro for load_ds (function)
macro_rules! Depcrate_segmentationload_ds {
() => {
// Module: crate::segmentation
// Provides: {"load_ds"}
// Dependencies: {}
# [doc = " Reload data segment register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn load_ds (sel : SegmentSelector) { asm ! ("movw {0:x}, %ds" , in (reg) sel . bits () , options (att_syntax)) ; }
};
}
