macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Queue < T > { }
    };
}

impl_16!();