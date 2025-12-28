macro_rules! deps {
    () => {
        IndexMap!();
        IntoIter!();
        IterMut!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a , K , V , S , const N : usize > IntoIterator for & 'a mut IndexMap < K , V , S , N > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_119!();