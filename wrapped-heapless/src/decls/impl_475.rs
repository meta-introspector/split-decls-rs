macro_rules! deps {
    () => {
        Consumer!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        unsafe impl < T > Send for Consumer < '_ , T > where T : Send { }
    };
}

impl_475!();