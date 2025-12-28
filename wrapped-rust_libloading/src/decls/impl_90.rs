macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        unsafe impl Send for Library { }
    };
}

impl_90!();