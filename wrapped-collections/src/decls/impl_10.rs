macro_rules! deps {
    () => {
        IIterator!();
        IIterable!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType > IntoIterator for IIterable < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
    };
}

impl_10!();