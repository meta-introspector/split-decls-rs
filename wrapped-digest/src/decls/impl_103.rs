macro_rules! deps {
    () => {
        InvalidOutputSize!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidOutputSize { }
    };
}

impl_103!();