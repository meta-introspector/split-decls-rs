macro_rules! deps {
    () => {
        IntoIter!();
        SmallVec!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'a , T , const N : usize > IntoIterator for & 'a mut SmallVec < T , N > { type IntoIter = core :: slice :: IterMut < 'a , T > ; type Item = & 'a mut T ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_141!()