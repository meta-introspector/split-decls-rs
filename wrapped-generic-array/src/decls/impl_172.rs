macro_rules! deps {
    () => {
        GenericArrayImplOdd!();
        Sealed!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < T , U > Sealed for GenericArrayImplOdd < T , U > { }
    };
}

impl_172!();