macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IAsyncOperation < TResult > { }
    };
}

impl_79!();