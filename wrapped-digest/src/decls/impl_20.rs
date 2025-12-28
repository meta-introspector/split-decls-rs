macro_rules! deps {
    () => {
        InvalidBufferSize!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidBufferSize { }
    };
}

impl_20!()