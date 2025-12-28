macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < T > Sealed for & mut T where T : Sealed { }
    };
}

impl_198!();