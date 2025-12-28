macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IAsyncActionWithProgress < TProgress > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({1f6db258-e803-48a1-9546-eb7353398884}") . push_slice (b";") . push_other (TProgress :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_61!();