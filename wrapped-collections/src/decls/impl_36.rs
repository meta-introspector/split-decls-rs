macro_rules! deps {
    () => {
        IMap!();
        IMap_Vtbl!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        unsafe impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: Interface for IMap < K , V > { type Vtable = IMap_Vtbl < K , V > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_36!();