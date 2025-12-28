macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T , U : ArrayLength > ConstDefault for GenericArray < T , U > where U :: ArrayType < T > : ConstDefault , { const DEFAULT : Self = Self { data : ConstDefault :: DEFAULT , } ; }
    };
}

impl_72!();