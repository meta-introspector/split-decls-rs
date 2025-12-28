macro_rules! deps {
    () => {
        OnceLock!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for OnceLock < T > { }
    };
}

impl_90!();