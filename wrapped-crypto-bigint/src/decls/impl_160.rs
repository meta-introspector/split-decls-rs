macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl ConstOne for Limb { const ONE : Self = Self :: ONE ; }
    };
}

impl_160!()