// Generated macro for outb (function)
macro_rules! Depcrate_iooutb {
() => {
// Module: crate::io
// Provides: {"outb"}
// Dependencies: {}
# [doc = " Write 8 bits to port"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs IO privileges."] # [inline] pub unsafe fn outb (port : u16 , val : u8) { asm ! ("outb %al, %dx" , in ("al") val , in ("dx") port , options (att_syntax)) ; }
};
}
