macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T , N : ArrayLength > IntoIterator for Box < GenericArray < T , N > > { type IntoIter = alloc :: vec :: IntoIter < T > ; type Item = T ; fn into_iter (self) -> Self :: IntoIter { GenericArray :: into_vec (self) . into_iter () } }
    };
}

impl_64!()