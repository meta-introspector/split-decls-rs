macro_rules! deps {
    () => {
        Bucket!();
        ParDrain!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < T : Send > IndexedParallelIterator for ParDrain < '_ , T > { indexed_parallel_iterator_methods ! (Bucket :: key) ; }
    };
}

impl_161!()