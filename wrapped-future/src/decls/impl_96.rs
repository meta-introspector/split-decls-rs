macro_rules! deps {
    () => {
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: imp :: CanInto < IAsyncInfo > for IAsyncOperationWithProgress < TResult , TProgress > { const QUERY : bool = true ; }
    };
}

impl_96!()