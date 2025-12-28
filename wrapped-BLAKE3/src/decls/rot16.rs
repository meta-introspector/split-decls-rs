macro_rules! rot16 {
    () => {
        # [inline (always)] fn rot16 (a : v128) -> v128 { v128_or (u32x4_shr (a , 16) , u32x4_shl (a , 32 - 16)) }
    };
}

rot16!()