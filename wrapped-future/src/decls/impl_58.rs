macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IAsyncActionWithProgress < TProgress > { }
    };
}

impl_58!();