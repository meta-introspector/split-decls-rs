macro_rules! deps {
    () => {
        IMapView!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IMapView < K , V > { }
    };
}

impl_47!();