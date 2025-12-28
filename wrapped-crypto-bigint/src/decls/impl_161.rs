macro_rules! deps {
    () => {
        Limb!();
        Zero!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl Zero for Limb { # [inline (always)] fn zero () -> Self { Self :: ZERO } }
    };
}

impl_161!();