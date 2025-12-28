macro_rules! deps {
    () => {
        IIterator!();
        IVector!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > IntoIterator for & IVector < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
    };
}

impl_67!();