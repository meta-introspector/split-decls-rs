macro_rules! deps {
    () => {
        MaxArrayLengthP1!();
        IsWithinUsizeBound!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < N > IsWithinUsizeBound for N where N : typenum :: IsLess < MaxArrayLengthP1 , Output = typenum :: consts :: True > { }
    };
}

impl_175!()