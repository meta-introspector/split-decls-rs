// Generated macro for ss (function)
macro_rules! Depcrate_segmentationss {
() => {
// Module: crate::segmentation
// Provides: {"ss"}
// Dependencies: {}
# [doc = " Returns the current value of the stack segment register."] pub fn ss () -> SegmentSelector { let segment : u16 ; unsafe { asm ! ("mov %ss, {0:x}" , out (reg) segment , options (att_syntax)) } ; SegmentSelector :: from_raw (segment) }
};
}
