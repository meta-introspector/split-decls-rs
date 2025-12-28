macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! POW10_LIMB {
    () => {
        deps!();
        # [cfg (fast_arithmetic = "64")] pub const POW10_LIMB : & [Limb] = & POW10_64 ;
    };
}

POW10_LIMB!()