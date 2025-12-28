macro_rules! deps {
    () => {
        Limb!();
        Zero!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl num_traits :: Zero for Limb { fn zero () -> Self { Self :: ZERO } fn is_zero (& self) -> bool { self . ct_eq (& Self :: ZERO) . into () } }
    };
}

impl_163!()