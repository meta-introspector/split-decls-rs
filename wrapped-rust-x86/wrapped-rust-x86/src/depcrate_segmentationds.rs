// Generated macro for ds (function)
macro_rules! Depcrate_segmentationds {
() => {
// Module: crate::segmentation
// Provides: {"ds"}
// Dependencies: {}
# [doc = " Returns the current value of the data segment register."] pub fn ds () -> SegmentSelector { let segment : u16 ; unsafe { asm ! ("mov %ds, {0:x}" , out (reg) segment , options (att_syntax)) } ; SegmentSelector :: from_raw (segment) }
};
}
