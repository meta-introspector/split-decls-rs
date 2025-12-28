macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < IAsyncInfo > for IAsyncOperation < TResult > { const QUERY : bool = true ; }
    };
}

impl_83!();