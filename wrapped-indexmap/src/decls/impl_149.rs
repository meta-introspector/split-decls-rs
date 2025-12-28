macro_rules! deps {
    () => {
        Bucket!();
        IntoParIter!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < T : Send > ParallelIterator for IntoParIter < T > { type Item = T ; parallel_iterator_methods ! (Bucket :: key) ; }
    };
}

impl_149!()