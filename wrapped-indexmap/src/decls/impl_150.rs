macro_rules! deps {
    () => {
        IntoParIter!();
        Bucket!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for IntoParIter < T > { indexed_parallel_iterator_methods ! (Bucket :: key) ; }
    };
}

impl_150!()