// Generated macro for inw (function)
macro_rules! Depcrate_ioinw {
() => {
// Module: crate::io
// Provides: {"inw"}
// Dependencies: {}
# [doc = " Read 16 bits from port"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs IO privileges."] # [inline] pub unsafe fn inw (port : u16) -> u16 { let ret : u16 ; asm ! ("inw %dx, %ax" , in ("dx") port , out ("ax") ret , options (att_syntax)) ; ret }
};
}
