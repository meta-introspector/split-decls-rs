macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IAsyncOperation < TResult > { }
    };
}

impl_80!()