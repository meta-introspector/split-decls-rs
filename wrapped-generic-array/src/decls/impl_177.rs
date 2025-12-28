macro_rules! deps {
    () => {
        ArrayLength!();
        IsWithinUsizeBound!();
        GenericArrayImplOdd!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        unsafe impl < N : ArrayLength > ArrayLength for UInt < N , B1 > where Self : IsWithinUsizeBound , { # [doc (hidden)] type ArrayType < T > = GenericArrayImplOdd < T , N :: ArrayType < T > > ; }
    };
}

impl_177!()