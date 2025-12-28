macro_rules! deps {
    () => {
        AsyncOperationCompletedHandler!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for AsyncOperationCompletedHandler < TResult > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({fcdcf02c-e5d8-4478-915a-4d90b74b83a5}") . push_slice (b";") . push_other (TResult :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_23!();