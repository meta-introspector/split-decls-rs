macro_rules! deps {
    () => {
        GetDisjointMutError!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl core :: error :: Error for GetDisjointMutError { }
    };
}

impl_199!()