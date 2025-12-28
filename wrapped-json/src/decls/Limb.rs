macro_rules! Limb {
    () => {
        # [cfg (fast_arithmetic = "64")] pub type Limb = u64 ;
    };
}

Limb!();