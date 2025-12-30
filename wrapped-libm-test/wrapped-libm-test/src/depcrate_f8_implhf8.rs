// Generated macro for hf8 (function)
macro_rules! Depcrate_f8_implhf8 {
() => {
// Module: crate::f8_impl
// Provides: {"hf8"}
// Dependencies: {}
pub const fn hf8 (s : & str) -> f8 { let Ok (bits) = libm :: support :: hex_float :: parse_hex_exact (s , 8 , 3) else { panic ! () } ; f8 (bits as u8) }
};
}
