macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < T > Sealed for [T ; 0] { }
    };
}

impl_143!();