macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        unsafe impl Sync for Library { }
    };
}

impl_91!();