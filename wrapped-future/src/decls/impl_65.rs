macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        unsafe impl < TProgress : windows_core :: RuntimeType + 'static > Sync for IAsyncActionWithProgress < TProgress > { }
    };
}

impl_65!();