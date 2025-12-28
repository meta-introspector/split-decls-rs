macro_rules! deps {
    () => {
        InvalidBufferSize!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidBufferSize { }
    };
}

impl_106!()