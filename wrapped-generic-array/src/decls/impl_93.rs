macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        unsafe impl < T : Zeroable , N : ArrayLength > Zeroable for GenericArray < T , N > { }
    };
}

impl_93!();