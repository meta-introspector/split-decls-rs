macro_rules! deps {
    () => {
        IMapView!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IMapView < K , V > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({e480ce40-a338-4ada-adcf-272272e48cb9}") . push_slice (b";") . push_other (K :: SIGNATURE) . push_slice (b";") . push_other (V :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_50!();