macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        unsafe impl Sync for Library { }
    };
}

impl_110!();