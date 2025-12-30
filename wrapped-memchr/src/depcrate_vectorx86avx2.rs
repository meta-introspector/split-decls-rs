// Generated macro for x86avx2 (module)
macro_rules! Depcrate_vectorx86avx2 {
() => {
// Module: crate::vector
// Provides: {"x86avx2"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] mod x86avx2 { use core :: arch :: x86_64 :: * ; use super :: { SensibleMoveMask , Vector } ; impl Vector for __m256i { const BYTES : usize = 32 ; const ALIGN : usize = Self :: BYTES - 1 ; type Mask = SensibleMoveMask ; # [inline (always)] unsafe fn splat (byte : u8) -> __m256i { _mm256_set1_epi8 (byte as i8) } # [inline (always)] unsafe fn load_aligned (data : * const u8) -> __m256i { _mm256_load_si256 (data as * const __m256i) } # [inline (always)] unsafe fn load_unaligned (data : * const u8) -> __m256i { _mm256_loadu_si256 (data as * const __m256i) } # [inline (always)] unsafe fn movemask (self) -> SensibleMoveMask { SensibleMoveMask (_mm256_movemask_epi8 (self) as u32) } # [inline (always)] unsafe fn cmpeq (self , vector2 : Self) -> __m256i { _mm256_cmpeq_epi8 (self , vector2) } # [inline (always)] unsafe fn and (self , vector2 : Self) -> __m256i { _mm256_and_si256 (self , vector2) } # [inline (always)] unsafe fn or (self , vector2 : Self) -> __m256i { _mm256_or_si256 (self , vector2) } } }
};
}
