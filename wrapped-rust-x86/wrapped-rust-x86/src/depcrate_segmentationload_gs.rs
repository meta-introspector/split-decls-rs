// Generated macro for load_gs (function)
macro_rules! Depcrate_segmentationload_gs {
() => {
// Module: crate::segmentation
// Provides: {"load_gs"}
// Dependencies: {}
# [doc = " Reload gs segment register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn load_gs (sel : SegmentSelector) { asm ! ("movw {0:x}, %gs" , in (reg) sel . bits () , options (att_syntax)) ; }
};
}
