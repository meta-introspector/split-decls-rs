macro_rules! deps {
    () => {
        ParValues!();
        Bucket!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Sync > ParallelIterator for ParValues < 'a , K , V > { type Item = & 'a V ; parallel_iterator_methods ! (Bucket :: value_ref) ; }
    };
}

impl_130!();