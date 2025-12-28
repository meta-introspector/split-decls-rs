macro_rules! deps {
    () => {
        IeeeFloat!();
        Semantics!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < S : Semantics > Neg for IeeeFloat < S > { type Output = Self ; fn neg (mut self) -> Self { self . read_only_sign_do_not_mutate = ! self . is_negative () ; self } }
    };
}

impl_39!();