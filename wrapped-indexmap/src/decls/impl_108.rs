macro_rules! deps {
    () => {
        Bucket!();
        ParIter!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < K : Sync , V : Sync > IndexedParallelIterator for ParIter < '_ , K , V > { indexed_parallel_iterator_methods ! (Bucket :: refs) ; }
    };
}

impl_108!()