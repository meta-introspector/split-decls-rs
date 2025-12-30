// Generated macro for inl (function)
macro_rules! Depcrate_ioinl {
() => {
// Module: crate::io
// Provides: {"inl"}
// Dependencies: {}
# [doc = " Read 32 bits from port"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs IO privileges."] # [inline] pub unsafe fn inl (port : u16) -> u32 { let ret : u32 ; asm ! ("inl %dx, %eax" , out ("eax") ret , in ("dx") port , options (att_syntax)) ; ret }
};
}
