macro_rules! deps {
    () => {
        ConstChoice!();
    };
}

macro_rules! u32_max {
    () => {
        deps!();
        # [doc = " `const` equivalent of `u32::max(a, b)`."] pub const fn u32_max (a : u32 , b : u32) -> u32 { ConstChoice :: from_u32_lt (a , b) . select_u32 (a , b) }
    };
}

u32_max!()