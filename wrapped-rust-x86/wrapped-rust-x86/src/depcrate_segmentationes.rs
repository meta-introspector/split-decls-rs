// Generated macro for es (function)
macro_rules! Depcrate_segmentationes {
() => {
// Module: crate::segmentation
// Provides: {"es"}
// Dependencies: {}
# [doc = " Returns the current value of the extra segment register."] pub fn es () -> SegmentSelector { let segment : u16 ; unsafe { asm ! ("mov %es, {0:x}" , out (reg) segment , options (att_syntax)) } ; SegmentSelector :: from_raw (segment) }
};
}
