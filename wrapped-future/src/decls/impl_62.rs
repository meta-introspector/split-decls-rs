macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < IAsyncInfo > for IAsyncActionWithProgress < TProgress > { const QUERY : bool = true ; }
    };
}

impl_62!();