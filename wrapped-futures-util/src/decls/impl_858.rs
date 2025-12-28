macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! impl_858 {
    () => {
        deps!();
        unsafe impl < Fut > Sync for Task < Fut > { }
    };
}

impl_858!();