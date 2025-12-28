macro_rules! deps {
    () => {
        IIterator!();
        IVector!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > IntoIterator for IVector < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
    };
}

impl_66!()