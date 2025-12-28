macro_rules! deps {
    () => {
        ExtendTarget!();
        Extend!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T : sealed :: Integer > Extend for T { fn extend < U > (self) -> U where T : ExtendTarget < U > , { sealed :: ExtendTargetSealed :: extend (self) } }
    };
}

impl_5!()