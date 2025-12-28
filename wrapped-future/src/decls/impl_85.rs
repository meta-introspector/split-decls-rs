macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static > Send for IAsyncOperation < TResult > { }
    };
}

impl_85!();