macro_rules! deps {
    () => {
        IsWithinUsizeBound!();
        ArrayLength!();
        GenericArrayImplEven!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        unsafe impl < N : ArrayLength > ArrayLength for UInt < N , B0 > where Self : IsWithinUsizeBound , { # [doc (hidden)] type ArrayType < T > = GenericArrayImplEven < T , N :: ArrayType < T > > ; }
    };
}

impl_176!()