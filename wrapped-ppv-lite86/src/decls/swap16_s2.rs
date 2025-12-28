macro_rules! swap16_s2 {
    () => {
        # [inline (always)] fn swap16_s2 (x : __m128i) -> __m128i { unsafe { _mm_shufflehi_epi16 (_mm_shufflelo_epi16 (x , 0b1011_0001) , 0b1011_0001) } }
    };
}

swap16_s2!()