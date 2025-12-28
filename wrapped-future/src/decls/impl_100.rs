macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: RuntimeName for IAsyncOperationWithProgress < TResult , TProgress > { const NAME : & 'static str = "Windows.Foundation.IAsyncOperationWithProgress" ; }
    };
}

impl_100!()