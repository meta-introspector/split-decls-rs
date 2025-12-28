macro_rules! deps {
    () => {
        IKeyValuePair!();
        IMap!();
        IIterator!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > IntoIterator for IMap < K , V > { type Item = IKeyValuePair < K , V > ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
    };
}

impl_40!()