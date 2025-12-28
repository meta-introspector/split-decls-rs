macro_rules! deps {
    () => {
        GenericArrayImplEven!();
        Sealed!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < T , U > Sealed for GenericArrayImplEven < T , U > { }
    };
}

impl_171!();