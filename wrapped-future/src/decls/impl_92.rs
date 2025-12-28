macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IAsyncOperationWithProgress < TResult , TProgress > { }
    };
}

impl_92!();