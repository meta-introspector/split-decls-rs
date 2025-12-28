macro_rules! deps {
    () => {
        InterruptHandle!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        unsafe impl Send for InterruptHandle { }
    };
}

impl_58!()