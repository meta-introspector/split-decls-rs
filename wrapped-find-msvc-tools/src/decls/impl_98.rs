macro_rules! deps {
    () => {
        Repr!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        unsafe impl Send for Repr { }
    };
}

impl_98!();