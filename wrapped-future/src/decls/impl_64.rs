macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        unsafe impl < TProgress : windows_core :: RuntimeType + 'static > Send for IAsyncActionWithProgress < TProgress > { }
    };
}

impl_64!();