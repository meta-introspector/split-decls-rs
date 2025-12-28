macro_rules! deps {
    () => {
        AttrList!();
        AList!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'a , A > IntoIterator for & 'a AttrList < A > { type Item = & 'a AList < A > ; type IntoIter = std :: slice :: Iter < 'a , AList < A > > ; fn into_iter (self) -> Self :: IntoIter { self . elems . iter () } }
    };
}

impl_44!();