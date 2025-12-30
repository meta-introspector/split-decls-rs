// Generated macro for load_tr (function)
macro_rules! Depcrate_taskload_tr {
() => {
// Module: crate::task
// Provides: {"load_tr"}
// Dependencies: {}
# [doc = " Loads the task register."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn load_tr (sel : segmentation :: SegmentSelector) { asm ! ("ltr {0:x}" , in (reg) sel . bits () , options (att_syntax , nostack , nomem , preserves_flags)) ; }
};
}
