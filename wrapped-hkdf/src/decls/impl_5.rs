macro_rules! deps {
    () => {
        InvalidLength!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidLength { }
    };
}

impl_5!()