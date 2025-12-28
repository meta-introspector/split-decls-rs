macro_rules! deps {
    () => {
        IMap!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IMap < K , V > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1}") . push_slice (b";") . push_other (K :: SIGNATURE) . push_slice (b";") . push_other (V :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_37!();