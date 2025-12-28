macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
        GenericArrayIter!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T , N : ArrayLength > IntoIterator for GenericArray < T , N > { type Item = T ; type IntoIter = GenericArrayIter < T , N > ; # [inline] fn into_iter (self) -> Self :: IntoIter { GenericArrayIter { array : ManuallyDrop :: new (self) , index : 0 , index_back : N :: USIZE , } } }
    };
}

impl_49!();