macro_rules! deps {
    () => {
        InvalidOutputSize!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidOutputSize { }
    };
}

impl_17!()