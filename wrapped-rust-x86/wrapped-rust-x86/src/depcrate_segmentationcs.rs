// Generated macro for cs (function)
macro_rules! Depcrate_segmentationcs {
() => {
// Module: crate::segmentation
// Provides: {"cs"}
// Dependencies: {}
# [doc = " Returns the current value of the code segment register."] pub fn cs () -> SegmentSelector { let segment : u16 ; unsafe { asm ! ("mov %cs, {0:x}" , out (reg) segment , options (att_syntax)) } ; SegmentSelector :: from_raw (segment) }
};
}
