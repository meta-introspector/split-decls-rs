// Generated macro for offsetz (function)
macro_rules! Depcrate_simd_neonoffsetz {
() => {
// Module: crate::simd::neon
// Provides: {"offsetz"}
// Dependencies: {}
# [inline] unsafe fn offsetz (x : uint8x16_t) -> u32 { offsetnz (vmvnq_u8 (x)) }
};
}
