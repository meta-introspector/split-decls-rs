macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T : Copy , N : ArrayLength > Copy for GenericArray < T , N > where N :: ArrayType < T > : Copy { }
    };
}

impl_26!();