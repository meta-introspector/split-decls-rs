macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArrayImplOdd!();
        IsWithinUsizeBound!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        unsafe impl < N : ArrayLength > ArrayLength for UInt < N , B1 > where Self : IsWithinUsizeBound , { # [doc (hidden)] type ArrayType < T > = GenericArrayImplOdd < T , N :: ArrayType < T > > ; }
    };
}

impl_31!()