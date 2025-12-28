macro_rules! deps {
    () => {
        Bucket!();
        ParIter!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Sync > ParallelIterator for ParIter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; parallel_iterator_methods ! (Bucket :: refs) ; }
    };
}

impl_107!()