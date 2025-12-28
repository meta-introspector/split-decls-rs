macro_rules! deps {
    () => {
        IMapView!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IMapView < K , V > { }
    };
}

impl_48!();