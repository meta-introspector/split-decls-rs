// Generated macro for inb (function)
macro_rules! Depcrate_ioinb {
() => {
// Module: crate::io
// Provides: {"inb"}
// Dependencies: {}
# [doc = " Read 8 bits from port"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs IO privileges."] # [inline] pub unsafe fn inb (port : u16) -> u8 { let ret : u8 ; asm ! ("inb %dx, %al" , in ("dx") port , out ("al") ret , options (att_syntax)) ; ret }
};
}
