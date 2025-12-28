macro_rules! deps {
    () => {
        IsWithinUsizeBound!();
        GenericArrayImplEven!();
        ArrayLength!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        unsafe impl < N : ArrayLength > ArrayLength for UInt < N , B0 > where Self : IsWithinUsizeBound , { # [doc (hidden)] type ArrayType < T > = GenericArrayImplEven < T , N :: ArrayType < T > > ; }
    };
}

impl_30!()