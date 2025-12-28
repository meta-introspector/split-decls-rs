macro_rules! deps {
    () => {
        InvalidLength!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidLength { }
    };
}

impl_32!()