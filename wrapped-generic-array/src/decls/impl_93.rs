macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        unsafe impl < T : Zeroable , N : ArrayLength > Zeroable for GenericArray < T , N > { }
    };
}

impl_93!()