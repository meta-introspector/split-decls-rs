// Generated macro for outl (function)
macro_rules! Depcrate_iooutl {
() => {
// Module: crate::io
// Provides: {"outl"}
// Dependencies: {}
# [doc = " Write 32 bits to port"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs IO privileges."] # [inline] pub unsafe fn outl (port : u16 , val : u32) { asm ! ("outl %eax, %dx" , in ("eax") val , in ("dx") port , options (att_syntax)) ; }
};
}
