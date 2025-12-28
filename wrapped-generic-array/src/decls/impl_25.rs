macro_rules! deps {
    () => {
        GenericArrayImplEven!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T , U > Sealed for GenericArrayImplEven < T , U > { }
    };
}

impl_25!()