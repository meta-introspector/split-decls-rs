macro_rules! deps {
    () => {
        ScopedJoinHandle!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        unsafe impl < T > Send for ScopedJoinHandle < '_ , T > { }
    };
}

impl_161!()