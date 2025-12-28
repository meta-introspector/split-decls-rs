macro_rules! deps {
    () => {
        IIterable!();
        IIterator!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType > IntoIterator for & IIterable < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
    };
}

impl_11!();