macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'a , T : 'a , N : ArrayLength > IntoIterator for & 'a mut GenericArray < T , N > { type IntoIter = slice :: IterMut < 'a , T > ; type Item = & 'a mut T ; # [inline] fn into_iter (self : & 'a mut GenericArray < T , N >) -> Self :: IntoIter { self . as_mut_slice () . iter_mut () } }
    };
}

impl_184!();