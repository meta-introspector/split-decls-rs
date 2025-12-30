// Generated macro for load_es (function)
macro_rules! Depcrate_segmentationload_es {
() => {
// Module: crate::segmentation
// Provides: {"load_es"}
// Dependencies: {}
# [doc = " Reload es segment register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn load_es (sel : SegmentSelector) { asm ! ("movw {0:x}, %es" , in (reg) sel . bits () , options (att_syntax)) ; }
};
}
