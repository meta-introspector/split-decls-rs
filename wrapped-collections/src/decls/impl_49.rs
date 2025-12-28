macro_rules! deps {
    () => {
        IMapView_Vtbl!();
        IMapView!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        unsafe impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: Interface for IMapView < K , V > { type Vtable = IMapView_Vtbl < K , V > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_49!()