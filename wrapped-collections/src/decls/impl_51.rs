macro_rules! deps {
    () => {
        IIterable!();
        IKeyValuePair!();
        IMapView!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < IIterable < IKeyValuePair < K , V > > > for IMapView < K , V > { const QUERY : bool = true ; }
    };
}

impl_51!();