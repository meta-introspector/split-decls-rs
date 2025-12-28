macro_rules! deps {
    () => {
        Bucket!();
        ParValues!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < K : Sync , V : Sync > IndexedParallelIterator for ParValues < '_ , K , V > { indexed_parallel_iterator_methods ! (Bucket :: value_ref) ; }
    };
}

impl_131!();