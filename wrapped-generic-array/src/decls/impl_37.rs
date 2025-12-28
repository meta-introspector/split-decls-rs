macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'a , T : 'a , N : ArrayLength > IntoIterator for & 'a GenericArray < T , N > { type IntoIter = slice :: Iter < 'a , T > ; type Item = & 'a T ; # [inline] fn into_iter (self : & 'a GenericArray < T , N >) -> Self :: IntoIter { self . as_slice () . iter () } }
    };
}

impl_37!()