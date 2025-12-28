macro_rules! deps {
    () => {
        Iter!();
        IntoIter!();
        FuturesUnordered!();
    };
}

macro_rules! impl_881 {
    () => {
        deps!();
        impl < 'a , Fut : Unpin > IntoIterator for & 'a FuturesUnordered < Fut > { type Item = & 'a Fut ; type IntoIter = Iter < 'a , Fut > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_881!();