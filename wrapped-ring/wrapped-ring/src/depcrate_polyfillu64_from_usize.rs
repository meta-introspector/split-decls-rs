// Generated macro for u64_from_usize (function)
macro_rules! Depcrate_polyfillu64_from_usize {
() => {
// Module: crate::polyfill
// Provides: {"u64_from_usize"}
// Dependencies: {}
# [inline (always)] pub const fn u64_from_usize (x : usize) -> u64 { # [allow (clippy :: cast_possible_truncation)] const _LOSSLESS : () = assert ! (usize :: MAX == ((usize :: MAX) as u64) as usize) ; x as u64 }
};
}
