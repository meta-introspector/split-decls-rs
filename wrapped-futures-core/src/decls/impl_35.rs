macro_rules! deps {
    () => {
        AtomicWaker!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        unsafe impl Sync for AtomicWaker { }
    };
}

impl_35!()