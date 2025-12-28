macro_rules! deps {
    () => {
        IVectorView!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IVectorView < T > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}") . push_slice (b";") . push_other (T :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_76!()