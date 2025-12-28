macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static > Sync for IAsyncOperation < TResult > { }
    };
}

impl_86!();