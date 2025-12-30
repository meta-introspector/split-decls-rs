// Generated macro for impl_180 (impl)
macro_rules! Depcrate_utf16_charimpl_180 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_180"}
// Dependencies: {}
impl Ord for Utf16Char { # [inline] fn cmp (& self , rhs : & Self) -> Ordering { let lhs = (self . units [0] as u32 , self . units [1] as u32) ; let rhs = (rhs . units [0] as u32 , rhs . units [1] as u32) ; let lhs = (lhs . 0 << (lhs . 1 >> 12)) + lhs . 1 ; let rhs = (rhs . 0 << (rhs . 1 >> 12)) + rhs . 1 ; lhs . cmp (& rhs) } }
};
}
