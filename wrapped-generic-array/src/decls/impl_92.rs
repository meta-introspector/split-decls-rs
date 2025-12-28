macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        unsafe impl < T : Pod , N : ArrayLength > Pod for GenericArray < T , N > where GenericArray < T , N > : Copy { }
    };
}

impl_92!()