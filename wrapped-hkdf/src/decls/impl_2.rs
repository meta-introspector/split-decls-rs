macro_rules! deps {
    () => {
        InvalidPrkLength!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidPrkLength { }
    };
}

impl_2!();