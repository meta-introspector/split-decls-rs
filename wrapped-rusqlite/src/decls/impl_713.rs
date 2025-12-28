macro_rules! deps {
    () => {
        InterruptHandle!();
    };
}

macro_rules! impl_713 {
    () => {
        deps!();
        unsafe impl Send for InterruptHandle { }
    };
}

impl_713!();