macro_rules! deps {
    () => {
        IMapView!();
        IIterator!();
        IKeyValuePair!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IntoIterator for IMapView < K , V > { type Item = IKeyValuePair < K , V > ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
    };
}

impl_53!()