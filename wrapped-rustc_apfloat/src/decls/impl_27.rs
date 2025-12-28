macro_rules! deps {
    () => {
        NegExt!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T : Neg < Output = Self > > NegExt for T { }
    };
}

impl_27!()