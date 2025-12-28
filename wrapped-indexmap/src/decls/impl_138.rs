macro_rules! deps {
    () => {
        ParValuesMut!();
        Bucket!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < K : Send , V : Send > IndexedParallelIterator for ParValuesMut < '_ , K , V > { indexed_parallel_iterator_methods ! (Bucket :: value_mut) ; }
    };
}

impl_138!()