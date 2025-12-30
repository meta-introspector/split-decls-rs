// Generated macro for offsetnz (function)
macro_rules! Depcrate_simd_neonoffsetnz {
() => {
// Module: crate::simd::neon
// Provides: {"offsetnz"}
// Dependencies: {}
# [inline] unsafe fn offsetnz (x : uint8x16_t) -> u32 { let x = vreinterpretq_u64_u8 (x) ; let low : u64 = vgetq_lane_u64 :: < 0 > (x) ; let high : u64 = vgetq_lane_u64 :: < 1 > (x) ; # [inline] fn clz (x : u64) -> u32 { for (i , b) in x . to_ne_bytes () . iter () . copied () . enumerate () { if b != 0 { return i as u32 ; } } 8 } if low != 0 { clz (low) } else if high != 0 { 8 + clz (high) } else { 16 } }
};
}
