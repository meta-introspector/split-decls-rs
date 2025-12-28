macro_rules! deps {
    () => {
        GenericArrayImplOdd!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T , U > Sealed for GenericArrayImplOdd < T , U > { }
    };
}

impl_26!()