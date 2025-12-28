macro_rules! deps {
    () => {
        AsyncActionWithProgressCompletedHandler!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for AsyncActionWithProgressCompletedHandler < TProgress > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({9c029f91-cc84-44fd-ac26-0a6c4e555281}") . push_slice (b";") . push_other (TProgress :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_16!()