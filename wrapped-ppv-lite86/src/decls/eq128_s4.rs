macro_rules! eq128_s4 {
    () => {
        # [allow (unused)] # [inline (always)] unsafe fn eq128_s4 (x : __m128i , y : __m128i) -> bool { let q = _mm_shuffle_epi32 (_mm_cmpeq_epi64 (x , y) , 0b1100_0110) ; _mm_cvtsi128_si64 (q) == - 1 }
    };
}

eq128_s4!();