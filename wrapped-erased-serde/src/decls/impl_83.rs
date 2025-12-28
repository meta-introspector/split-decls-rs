macro_rules! deps {
    () => {
        Serialize!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < T > sealed :: serialize :: Sealed for T where T : ? Sized + serde :: Serialize { }
    };
}

impl_83!();