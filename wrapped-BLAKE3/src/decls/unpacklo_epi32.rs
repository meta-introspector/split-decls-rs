macro_rules! unpacklo_epi32 {
    () => {
        # [inline (always)] fn unpacklo_epi32 (a : v128 , b : v128) -> v128 { i32x4_shuffle :: < 0 , 4 , 1 , 5 > (a , b) }
    };
}

unpacklo_epi32!();