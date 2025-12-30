// Generated macro for nib2byte_avx2 (function)
macro_rules! Depcrate_decodenib2byte_avx2 {
() => {
// Module: crate::decode
// Provides: {"nib2byte_avx2"}
// Dependencies: {}
# [inline] # [target_feature (enable = "avx2")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] unsafe fn nib2byte_avx2 (a1 : __m256i , b1 : __m256i , a2 : __m256i , b2 : __m256i) -> __m256i { let a4_1 = _mm256_slli_epi16 (a1 , 4) ; let a4_2 = _mm256_slli_epi16 (a2 , 4) ; let a4orb_1 = _mm256_or_si256 (a4_1 , b1) ; let a4orb_2 = _mm256_or_si256 (a4_2 , b2) ; let pck1 = _mm256_packus_epi16 (a4orb_1 , a4orb_2) ; _mm256_permute4x64_epi64 (pck1 , _0213) }
};
}
