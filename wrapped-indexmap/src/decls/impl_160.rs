macro_rules! deps {
    () => {
        Bucket!();
        ParDrain!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < T : Send > ParallelIterator for ParDrain < '_ , T > { type Item = T ; parallel_iterator_methods ! (Bucket :: key) ; }
    };
}

impl_160!()