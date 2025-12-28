macro_rules! deps {
    () => {
        Vector!();
        SensibleMoveMask!();
        Mask!();
    };
}

macro_rules! x86sse2 {
    () => {
        deps!();
        # [cfg (target_arch = "x86_64")] mod x86sse2 { use core :: arch :: x86_64 :: * ; use super :: { SensibleMoveMask , Vector } ; impl Vector for __m128i { const BYTES : usize = 16 ; const ALIGN : usize = Self :: BYTES - 1 ; type Mask = SensibleMoveMask ; # [inline (always)] unsafe fn splat (byte : u8) -> __m128i { _mm_set1_epi8 (byte as i8) } # [inline (always)] unsafe fn load_aligned (data : * const u8) -> __m128i { _mm_load_si128 (data as * const __m128i) } # [inline (always)] unsafe fn load_unaligned (data : * const u8) -> __m128i { _mm_loadu_si128 (data as * const __m128i) } # [inline (always)] unsafe fn movemask (self) -> SensibleMoveMask { SensibleMoveMask (_mm_movemask_epi8 (self) as u32) } # [inline (always)] unsafe fn cmpeq (self , vector2 : Self) -> __m128i { _mm_cmpeq_epi8 (self , vector2) } # [inline (always)] unsafe fn and (self , vector2 : Self) -> __m128i { _mm_and_si128 (self , vector2) } # [inline (always)] unsafe fn or (self , vector2 : Self) -> __m128i { _mm_or_si128 (self , vector2) } } }
    };
}

x86sse2!()