macro_rules! deps {
    () => {
        Collector!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        unsafe impl Send for Collector { }
    };
}

impl_67!()