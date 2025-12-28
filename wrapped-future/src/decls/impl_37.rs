macro_rules! deps {
    () => {
        AsyncOperationWithProgressCompletedHandler!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: RuntimeType for AsyncOperationWithProgressCompletedHandler < TResult , TProgress > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({e85df41d-6aa7-46e3-a8e2-f009d840c627}") . push_slice (b";") . push_other (TResult :: SIGNATURE) . push_slice (b";") . push_other (TProgress :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_37!()