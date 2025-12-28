macro_rules! loadu_si128 {
    () => {
        # [doc = " Safe equivalent to `_mm_loadu_si128` for byte slices."] # [must_use] # [inline (always)] fn loadu_si128 (src : & [u8]) -> __m128i { assert_eq ! (src . len () , size_of ::< __m128i > ()) ; unsafe { _mm_loadu_si128 (src . as_ptr () . cast :: < __m128i > ()) } }
    };
}

loadu_si128!()