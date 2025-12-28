macro_rules! deps {
    () => {
        IntoIter!();
        FuturesUnordered!();
        IterMut!();
    };
}

macro_rules! impl_882 {
    () => {
        deps!();
        impl < 'a , Fut : Unpin > IntoIterator for & 'a mut FuturesUnordered < Fut > { type Item = & 'a mut Fut ; type IntoIter = IterMut < 'a , Fut > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_882!();