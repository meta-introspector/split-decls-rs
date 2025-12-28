macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        unsafe impl Sync for Library { }
    };
}

impl_153!()