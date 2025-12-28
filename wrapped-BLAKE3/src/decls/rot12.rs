macro_rules! rot12 {
    () => {
        # [inline (always)] fn rot12 (a : v128) -> v128 { v128_or (u32x4_shr (a , 12) , u32x4_shl (a , 32 - 12)) }
    };
}

rot12!();