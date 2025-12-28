macro_rules! deps {
    () => {
        ScopedJoinHandle!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        unsafe impl < T > Sync for ScopedJoinHandle < '_ , T > { }
    };
}

impl_162!();