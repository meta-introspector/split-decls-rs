macro_rules! deps {
    () => {
        AList!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a , A > IntoIterator for & 'a AList < A > { type Item = & 'a A ; type IntoIter = std :: slice :: Iter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . elems . iter () } }
    };
}

impl_53!();