// Generated macro for bit_set (function)
macro_rules! Depcrate_simd_neonbit_set {
() => {
// Module: crate::simd::neon
// Provides: {"bit_set"}
// Dependencies: {}
const fn bit_set (x : u8) -> bool { matches ! (x , b'0' ..= b'9' | b'a' ..= b'z' | b'A' ..= b'Z' | b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~') }
};
}
