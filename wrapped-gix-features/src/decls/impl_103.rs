macro_rules! deps {
    () => {
        Compress!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        unsafe impl Send for Compress { }
    };
}

impl_103!()