// Generated macro for gs (function)
macro_rules! Depcrate_segmentationgs {
() => {
// Module: crate::segmentation
// Provides: {"gs"}
// Dependencies: {}
# [doc = " Returns the current value of the GS segment register."] pub fn gs () -> SegmentSelector { let segment : u16 ; unsafe { asm ! ("mov %gs, {0:x}" , out (reg) segment , options (att_syntax)) } ; SegmentSelector :: from_raw (segment) }
};
}
