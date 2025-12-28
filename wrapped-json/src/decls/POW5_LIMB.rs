macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! POW5_LIMB {
    () => {
        deps!();
        # [cfg (fast_arithmetic = "64")] pub const POW5_LIMB : & [Limb] = & POW5_64 ;
    };
}

POW5_LIMB!();