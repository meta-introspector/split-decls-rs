macro_rules! deps {
    () => {
        Interface!();
        Weak!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        unsafe impl < I : Interface > Send for Weak < I > { }
    };
}

impl_212!();