macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IAsyncActionWithProgress < TProgress > { }
    };
}

impl_59!()