macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl zeroize :: DefaultIsZeroes for Limb { }
    };
}

impl_172!();