macro_rules! deps {
    () => {
        AtomicWaker!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        unsafe impl Send for AtomicWaker { }
    };
}

impl_34!();