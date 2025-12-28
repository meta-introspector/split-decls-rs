macro_rules! hash {
    () => {
        # [cfg (target_pointer_width = "64")] # [inline] fn hash (key : usize , bits : u32) -> usize { key . wrapping_mul (0x9E3779B97F4A7C15) >> (64 - bits) }
    };
}

hash!();