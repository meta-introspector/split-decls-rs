macro_rules! deps {
    () => {
        Bucket!();
        IntoParIter!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < K : Send , V : Send > IndexedParallelIterator for IntoParIter < K , V > { indexed_parallel_iterator_methods ! (Bucket :: key_value) ; }
    };
}

impl_101!();