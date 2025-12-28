macro_rules! deps {
    () => {
        CapacityError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl core :: error :: Error for CapacityError { }
    };
}

impl_28!()