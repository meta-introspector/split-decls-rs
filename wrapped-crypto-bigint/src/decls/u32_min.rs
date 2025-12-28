macro_rules! deps {
    () => {
        ConstChoice!();
    };
}

macro_rules! u32_min {
    () => {
        deps!();
        # [doc = " `const` equivalent of `u32::min(a, b)`."] pub const fn u32_min (a : u32 , b : u32) -> u32 { ConstChoice :: from_u32_lt (a , b) . select_u32 (b , a) }
    };
}

u32_min!()