// Generated macro for match_tail (function)
macro_rules! Depcrate_simd_swarmatch_tail {
() => {
// Module: crate::simd::swar
// Provides: {"match_tail"}
// Dependencies: {}
# [cold] # [inline] fn match_tail (f : impl Fn (u8) -> bool , bytes : & [u8]) -> usize { for (i , & b) in bytes . iter () . enumerate () { if ! f (b) { return i ; } } bytes . len () }
};
}
