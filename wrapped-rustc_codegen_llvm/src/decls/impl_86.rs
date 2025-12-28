macro_rules! deps {
    () => {
        ThinData!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        unsafe impl Sync for ThinData { }
    };
}

impl_86!();