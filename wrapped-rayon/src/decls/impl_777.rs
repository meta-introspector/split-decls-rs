macro_rules! deps {
    () => {
        ProductConsumer!();
    };
}

macro_rules! impl_777 {
    () => {
        deps!();
        unsafe impl < P : Send > Send for ProductConsumer < P > { }
    };
}

impl_777!();