// Generated macro for tr (function)
macro_rules! Depcrate_tasktr {
() => {
// Module: crate::task
// Provides: {"tr"}
// Dependencies: {}
# [doc = " Returns the current value of the task register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn tr () -> segmentation :: SegmentSelector { let segment : u16 ; asm ! ("str {0:x}" , out (reg) segment , options (att_syntax , nostack , nomem , preserves_flags)) ; segmentation :: SegmentSelector :: from_raw (segment) }
};
}
