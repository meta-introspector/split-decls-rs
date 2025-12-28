macro_rules! deps {
    () => {
        IAsyncOperation!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IAsyncOperation < TResult > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2}") . push_slice (b";") . push_other (TResult :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_82!();