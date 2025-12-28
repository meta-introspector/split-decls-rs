macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        unsafe impl < 'h > Sync for Iter < 'h > { }
    };
}

impl_134!();