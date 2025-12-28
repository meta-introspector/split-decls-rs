macro_rules! deps {
    () => {
        EnvWrapper!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        unsafe impl Sync for EnvWrapper { }
    };
}

impl_259!();