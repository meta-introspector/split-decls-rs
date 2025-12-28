macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IAsyncActionWithProgress < TProgress > { const NAME : & 'static str = "Windows.Foundation.IAsyncActionWithProgress" ; }
    };
}

impl_66!();