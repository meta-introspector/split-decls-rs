macro_rules! deps {
    () => {
        Truncate!();
        TruncateTarget!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T : sealed :: Integer > Truncate for T { fn truncate < U > (self) -> U where T : TruncateTarget < U > , { sealed :: TruncateTargetSealed :: truncate (self) } }
    };
}

impl_7!();