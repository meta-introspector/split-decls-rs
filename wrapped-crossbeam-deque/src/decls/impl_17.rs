macro_rules! deps {
    () => {
        Stealer!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Stealer < T > { }
    };
}

impl_17!();