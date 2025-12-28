macro_rules! rot7 {
    () => {
        # [inline (always)] fn rot7 (a : v128) -> v128 { v128_or (u32x4_shr (a , 7) , u32x4_shl (a , 32 - 7)) }
    };
}

rot7!();