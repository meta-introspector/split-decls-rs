macro_rules! deps {
    () => {
        Sealed!();
        GenericArrayImplEven!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < T , U > Sealed for GenericArrayImplEven < T , U > { }
    };
}

impl_171!()