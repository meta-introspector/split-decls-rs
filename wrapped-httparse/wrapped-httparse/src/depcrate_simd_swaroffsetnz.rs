// Generated macro for offsetnz (function)
macro_rules! Depcrate_simd_swaroffsetnz {
() => {
// Module: crate::simd::swar
// Provides: {"offsetnz"}
// Dependencies: {}
# [doc = " Check block to find offset of first non-zero byte"] # [inline] fn offsetnz (block : usize) -> usize { if block == 0 { return BLOCK_SIZE ; } for (i , b) in block . to_ne_bytes () . iter () . copied () . enumerate () { if b != 0 { return i ; } } unreachable ! () }
};
}
