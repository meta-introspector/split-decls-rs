macro_rules! rot8 {
    () => {
        # [inline (always)] fn rot8 (a : v128) -> v128 { v128_or (u32x4_shr (a , 8) , u32x4_shl (a , 32 - 8)) }
    };
}

rot8!()