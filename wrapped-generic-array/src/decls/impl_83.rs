macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < Z : Zeroize , N : ArrayLength > Zeroize for GenericArray < Z , N > { fn zeroize (& mut self) { self . as_mut_slice () . iter_mut () . zeroize () } }
    };
}

impl_83!()