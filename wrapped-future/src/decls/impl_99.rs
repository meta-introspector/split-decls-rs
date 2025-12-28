macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > Sync for IAsyncOperationWithProgress < TResult , TProgress > { }
    };
}

impl_99!();