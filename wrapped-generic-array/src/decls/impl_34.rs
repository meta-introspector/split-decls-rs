macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        unsafe impl < T : Sync , N : ArrayLength > Sync for GenericArray < T , N > { }
    };
}

impl_34!()