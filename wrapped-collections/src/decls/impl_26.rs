macro_rules! deps {
    () => {
        IKeyValuePair_Vtbl!();
        IKeyValuePair!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        unsafe impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: Interface for IKeyValuePair < K , V > { type Vtable = IKeyValuePair_Vtbl < K , V > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_26!()