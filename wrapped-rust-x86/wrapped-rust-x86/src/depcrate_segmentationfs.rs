// Generated macro for fs (function)
macro_rules! Depcrate_segmentationfs {
() => {
// Module: crate::segmentation
// Provides: {"fs"}
// Dependencies: {}
# [doc = " Returns the current value of the FS segment register."] pub fn fs () -> SegmentSelector { let segment : u16 ; unsafe { asm ! ("mov %fs, {0:x}" , out (reg) segment , options (att_syntax)) } ; SegmentSelector :: from_raw (segment) }
};
}
