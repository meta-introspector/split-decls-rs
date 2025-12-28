macro_rules! unpacklo_epi64 {
    () => {
        # [inline (always)] fn unpacklo_epi64 (a : v128 , b : v128) -> v128 { i64x2_shuffle :: < 0 , 2 > (a , b) }
    };
}

unpacklo_epi64!();