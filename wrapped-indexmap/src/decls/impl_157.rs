macro_rules! deps {
    () => {
        ParIter!();
        Bucket!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < T : Sync > IndexedParallelIterator for ParIter < '_ , T > { indexed_parallel_iterator_methods ! (Bucket :: key_ref) ; }
    };
}

impl_157!();