// Generated macro for usize_from_u32 (function)
macro_rules! Depcrate_polyfillusize_from_u32 {
() => {
// Module: crate::polyfill
// Provides: {"usize_from_u32"}
// Dependencies: {}
pub const fn usize_from_u32 (x : u32) -> usize { # [allow (clippy :: cast_possible_truncation)] const _LOSSLESS : () = assert ! (u32 :: MAX == ((u32 :: MAX) as usize) as u32) ; x as usize }
};
}
