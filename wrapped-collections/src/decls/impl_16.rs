macro_rules! deps {
    () => {
        IIterator!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IIterator < T > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({6a79e863-4300-459a-9966-cbb660963ee1}") . push_slice (b";") . push_other (T :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_16!()