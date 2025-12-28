macro_rules! deps {
    () => {
        Interface!();
        Weak!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        unsafe impl < I : Interface > Sync for Weak < I > { }
    };
}

impl_213!()