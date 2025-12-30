// Generated macro for unhex_avx2 (function)
macro_rules! Depcrate_decodeunhex_avx2 {
() => {
// Module: crate::decode
// Provides: {"unhex_avx2"}
// Dependencies: {}
# [inline] # [target_feature (enable = "avx2")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] unsafe fn unhex_avx2 (value : __m256i) -> __m256i { let sr6 = _mm256_srai_epi16 (value , 6) ; let and15 = _mm256_and_si256 (value , _mm256_set1_epi16 (0xf)) ; let mul = _mm256_maddubs_epi16 (sr6 , _mm256_set1_epi16 (9)) ; _mm256_add_epi16 (mul , and15) }
};
}
