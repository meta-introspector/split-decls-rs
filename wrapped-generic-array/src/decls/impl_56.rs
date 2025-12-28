macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArrayIter!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T , N : ArrayLength > FusedIterator for GenericArrayIter < T , N > { }
    };
}

impl_56!();