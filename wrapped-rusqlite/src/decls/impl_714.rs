macro_rules! deps {
    () => {
        InterruptHandle!();
    };
}

macro_rules! impl_714 {
    () => {
        deps!();
        unsafe impl Sync for InterruptHandle { }
    };
}

impl_714!();