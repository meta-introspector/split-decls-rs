macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        unsafe impl Send for List { }
    };
}

impl_103!()