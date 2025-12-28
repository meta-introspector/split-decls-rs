macro_rules! deps {
    () => {
        IntoIter!();
        SmallVec!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'a , T , const N : usize > IntoIterator for & 'a SmallVec < T , N > { type IntoIter = core :: slice :: Iter < 'a , T > ; type Item = & 'a T ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_140!();