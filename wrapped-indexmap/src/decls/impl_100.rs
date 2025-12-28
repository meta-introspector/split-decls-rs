macro_rules! deps {
    () => {
        IntoParIter!();
        Bucket!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < K : Send , V : Send > ParallelIterator for IntoParIter < K , V > { type Item = (K , V) ; parallel_iterator_methods ! (Bucket :: key_value) ; }
    };
}

impl_100!();