macro_rules! deps {
    () => {
        ParKeys!();
        Bucket!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < K : Sync , V : Sync > IndexedParallelIterator for ParKeys < '_ , K , V > { indexed_parallel_iterator_methods ! (Bucket :: key_ref) ; }
    };
}

impl_126!();