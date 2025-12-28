macro_rules! deps {
    () => {
        IKeyValuePair!();
        IIterator!();
        IMap!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IntoIterator for & IMap < K , V > { type Item = IKeyValuePair < K , V > ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
    };
}

impl_41!();