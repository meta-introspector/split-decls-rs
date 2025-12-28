macro_rules! deps {
    () => {
        Parker!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        unsafe impl Send for Parker { }
    };
}

impl_95!()