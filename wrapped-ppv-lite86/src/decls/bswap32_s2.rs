macro_rules! bswap32_s2 {
    () => {
        # [inline (always)] fn bswap32_s2 (x : __m128i) -> __m128i { unsafe { let mut y = _mm_unpacklo_epi8 (x , _mm_setzero_si128 ()) ; y = _mm_shufflehi_epi16 (y , 0b0001_1011) ; y = _mm_shufflelo_epi16 (y , 0b0001_1011) ; let mut z = _mm_unpackhi_epi8 (x , _mm_setzero_si128 ()) ; z = _mm_shufflehi_epi16 (z , 0b0001_1011) ; z = _mm_shufflelo_epi16 (z , 0b0001_1011) ; _mm_packus_epi16 (y , z) } }
    };
}

bswap32_s2!()