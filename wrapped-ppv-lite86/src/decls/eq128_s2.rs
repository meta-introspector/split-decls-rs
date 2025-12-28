macro_rules! eq128_s2 {
    () => {
        # [inline (always)] unsafe fn eq128_s2 (x : __m128i , y : __m128i) -> bool { let q = _mm_cmpeq_epi32 (x , y) ; let p = _mm_cvtsi128_si64 (_mm_srli_si128 (q , 8)) ; let q = _mm_cvtsi128_si64 (q) ; (p & q) == - 1 }
    };
}

eq128_s2!()