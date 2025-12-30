// Generated macro for load_cs (function)
macro_rules! Depcrate_bits64_segmentationload_cs {
() => {
// Module: crate::bits64::segmentation
// Provides: {"load_cs"}
// Dependencies: {}
# [doc = " Reload code segment register."] # [doc = ""] # [doc = " Note this is special since we can not directly move"] # [doc = " to %cs. Instead we push the new segment selector"] # [doc = " and return value on the stack and use lretq"] # [doc = " to reload cs and continue at 1:."] # [doc = ""] # [doc = " # Safety"] # [doc = " Can cause a GP-fault with a bad `sel` value."] # [cfg (target_arch = "x86_64")] pub unsafe fn load_cs (sel : SegmentSelector) { asm ! ("pushq {0}; \
          leaq  1f(%rip), %rax; \
          pushq %rax; \
          lretq; \
          1:" , in (reg) sel . bits () as usize , out ("rax") _ , options (att_syntax)) ; }
};
}
