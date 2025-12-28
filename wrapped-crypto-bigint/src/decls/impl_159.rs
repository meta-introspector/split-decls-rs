macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl ConstZero for Limb { const ZERO : Self = Self :: ZERO ; }
    };
}

impl_159!();