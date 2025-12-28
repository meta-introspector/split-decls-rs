macro_rules! deps {
    () => {
        Bucket!();
        ParIterMut!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < K : Sync + Send , V : Send > IndexedParallelIterator for ParIterMut < '_ , K , V > { indexed_parallel_iterator_methods ! (Bucket :: ref_mut) ; }
    };
}

impl_114!();