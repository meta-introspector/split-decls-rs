macro_rules! deps {
    () => {
        AsyncActionProgressHandler!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for AsyncActionProgressHandler < TProgress > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({6d844858-0cff-4590-ae89-95a5a5c8b4b8}") . push_slice (b";") . push_other (TProgress :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_9!();