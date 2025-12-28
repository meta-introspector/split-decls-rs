macro_rules! deps {
    () => {
        InterruptHandle!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        unsafe impl Sync for InterruptHandle { }
    };
}

impl_59!()