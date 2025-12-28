macro_rules! deps {
    () => {
        IIterator!();
        IKeyValuePair!();
        IMapView!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IntoIterator for & IMapView < K , V > { type Item = IKeyValuePair < K , V > ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
    };
}

impl_54!();