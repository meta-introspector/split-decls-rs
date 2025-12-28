macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T : Copy , N : ArrayLength > Copy for GenericArray < T , N > where N :: ArrayType < T > : Copy { }
    };
}

impl_26!()