macro_rules! deps {
    () => {
        IMapView!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IMapView < K , V > { const NAME : & 'static str = "Windows.Foundation.Collections.IMapView" ; }
    };
}

impl_55!()