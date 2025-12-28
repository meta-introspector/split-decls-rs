macro_rules! deps {
    () => {
        Receiver!();
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'a , T > IntoIterator for & 'a Receiver < T > { type Item = T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_27!()