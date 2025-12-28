macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        unsafe impl < T : Sync , N : ArrayLength > Sync for GenericArray < T , N > { }
    };
}

impl_180!();