macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > Send for IAsyncOperationWithProgress < TResult , TProgress > { }
    };
}

impl_98!()