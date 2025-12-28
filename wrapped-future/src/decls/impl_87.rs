macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IAsyncOperation < TResult > { const NAME : & 'static str = "Windows.Foundation.IAsyncOperation" ; }
    };
}

impl_87!()