macro_rules! deps {
    () => {
        One!();
        Limb!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl num_traits :: One for Limb { fn one () -> Self { Self :: ONE } fn is_one (& self) -> bool { self . ct_eq (& Self :: ONE) . into () } }
    };
}

impl_164!()