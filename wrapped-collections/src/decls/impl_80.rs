macro_rules! deps {
    () => {
        IVectorView!();
        IIterator!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > IntoIterator for & IVectorView < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
    };
}

impl_80!();