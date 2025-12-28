macro_rules! deps {
    () => {
        Bucket!();
        ParDrain!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < K : Send , V : Send > IndexedParallelIterator for ParDrain < '_ , K , V > { indexed_parallel_iterator_methods ! (Bucket :: key_value) ; }
    };
}

impl_118!();