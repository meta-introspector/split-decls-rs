macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! split_u64 {
    () => {
        deps!();
        # [doc = " Split u64 into limbs, in little-endian order."] # [inline] # [cfg (fast_arithmetic = "64")] fn split_u64 (x : u64) -> [Limb ; 1] { [as_limb (x)] }
    };
}

split_u64!();