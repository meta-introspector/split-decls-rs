macro_rules! deps {
    () => {
        IKeyValuePair!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IKeyValuePair < K , V > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500}") . push_slice (b";") . push_other (K :: SIGNATURE) . push_slice (b";") . push_other (V :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_27!()