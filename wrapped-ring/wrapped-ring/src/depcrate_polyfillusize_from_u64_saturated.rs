// Generated macro for usize_from_u64_saturated (function)
macro_rules! Depcrate_polyfillusize_from_u64_saturated {
() => {
// Module: crate::polyfill
// Provides: {"usize_from_u64_saturated"}
// Dependencies: {}
# [doc = " const-capable `x.try_into().unwrap_or(usize::MAX)`"] # [allow (clippy :: cast_possible_truncation)] # [inline (always)] pub const fn usize_from_u64_saturated (x : u64) -> usize { const USIZE_MAX : u64 = u64_from_usize (usize :: MAX) ; if x < USIZE_MAX { x as usize } else { usize :: MAX } }
};
}
