macro_rules! deps {
    () => {
        IIterable!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IIterable < T > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({faa585ea-6214-4217-afda-7f46de5869b3}") . push_slice (b";") . push_other (T :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_4!();