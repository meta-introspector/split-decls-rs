macro_rules! deps {
    () => {
        CastTo!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < T > CastTo for T { type Target = T ; }
    };
}

impl_134!();