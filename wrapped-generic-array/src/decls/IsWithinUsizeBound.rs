macro_rules! deps {
    () => {
        MaxArrayLengthP1!();
    };
}

macro_rules! IsWithinUsizeBound {
    () => {
        deps!();
        # [doc = " Helper trait to hide the complex bound under a simpler name"] trait IsWithinUsizeBound : typenum :: IsLess < MaxArrayLengthP1 , Output = typenum :: consts :: True > { }
    };
}

IsWithinUsizeBound!()