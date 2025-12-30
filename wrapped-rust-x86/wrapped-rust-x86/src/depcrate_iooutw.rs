// Generated macro for outw (function)
macro_rules! Depcrate_iooutw {
() => {
// Module: crate::io
// Provides: {"outw"}
// Dependencies: {}
# [doc = " Write 16 bits to port"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs IO privileges."] # [inline] pub unsafe fn outw (port : u16 , val : u16) { asm ! ("outw %ax, %dx" , in ("ax") val , in ("dx") port , options (att_syntax)) ; }
};
}
