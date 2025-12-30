// Generated macro for load_cs (function)
macro_rules! Depcrate_bits32_segmentationload_cs {
() => {
// Module: crate::bits32::segmentation
// Provides: {"load_cs"}
// Dependencies: {}
# [doc = " Reload code segment register."] # [doc = " Note this is special since we can not directly move"] # [doc = " to %cs. Instead we push the new segment selector"] # [doc = " and return value on the stack and use lretl"] # [doc = " to reload cs and continue at 1:."] # [cfg (target_arch = "x86")] pub unsafe fn load_cs (sel : SegmentSelector) { asm ! ("pushl {0}; \
          pushl $1f; \
          lretl; \
          1:" , in (reg) sel . bits () as u32 , options (att_syntax)) ; }
};
}
