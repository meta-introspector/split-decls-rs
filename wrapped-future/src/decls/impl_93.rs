macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IAsyncOperationWithProgress < TResult , TProgress > { }
    };
}

impl_93!();