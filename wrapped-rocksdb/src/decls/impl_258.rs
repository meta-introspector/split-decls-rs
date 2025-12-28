macro_rules! deps {
    () => {
        EnvWrapper!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        unsafe impl Send for EnvWrapper { }
    };
}

impl_258!()