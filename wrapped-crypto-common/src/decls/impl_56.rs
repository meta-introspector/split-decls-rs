macro_rules! deps {
    () => {
        InvalidLength!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidLength { }
    };
}

impl_56!()