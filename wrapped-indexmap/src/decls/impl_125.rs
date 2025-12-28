macro_rules! deps {
    () => {
        ParKeys!();
        Bucket!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Sync > ParallelIterator for ParKeys < 'a , K , V > { type Item = & 'a K ; parallel_iterator_methods ! (Bucket :: key_ref) ; }
    };
}

impl_125!()