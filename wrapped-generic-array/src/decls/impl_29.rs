macro_rules! deps {
    () => {
        IsWithinUsizeBound!();
        MaxArrayLengthP1!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < N > IsWithinUsizeBound for N where N : typenum :: IsLess < MaxArrayLengthP1 , Output = typenum :: consts :: True > { }
    };
}

impl_29!()