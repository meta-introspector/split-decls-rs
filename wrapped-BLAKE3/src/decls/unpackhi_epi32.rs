macro_rules! unpackhi_epi32 {
    () => {
        # [inline (always)] fn unpackhi_epi32 (a : v128 , b : v128) -> v128 { i32x4_shuffle :: < 2 , 6 , 3 , 7 > (a , b) }
    };
}

unpackhi_epi32!();