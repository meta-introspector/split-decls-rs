macro_rules! deps {
    () => {
        GetDisjointMutError!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl core :: error :: Error for GetDisjointMutError { }
    };
}

impl_23!()