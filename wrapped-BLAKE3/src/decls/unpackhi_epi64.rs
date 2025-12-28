macro_rules! unpackhi_epi64 {
    () => {
        # [inline (always)] fn unpackhi_epi64 (a : v128 , b : v128) -> v128 { i64x2_shuffle :: < 1 , 3 > (a , b) }
    };
}

unpackhi_epi64!()