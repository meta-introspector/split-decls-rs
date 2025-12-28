macro_rules! deps {
    () => {
        WrappedWaker!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        unsafe impl Sync for WrappedWaker { }
    };
}

impl_529!()