macro_rules! deps {
    () => {
        Stealer!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Stealer < T > { }
    };
}

impl_18!();