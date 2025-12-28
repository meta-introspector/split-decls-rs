macro_rules! deps {
    () => {
        Receiver!();
        IntoIter!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > IntoIterator for Receiver < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { receiver : self } } }
    };
}

impl_28!();