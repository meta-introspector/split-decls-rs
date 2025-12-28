macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        unsafe impl Send for Library { }
    };
}

impl_152!()