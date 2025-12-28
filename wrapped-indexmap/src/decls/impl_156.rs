macro_rules! deps {
    () => {
        ParIter!();
        Bucket!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < 'a , T : Sync > ParallelIterator for ParIter < 'a , T > { type Item = & 'a T ; parallel_iterator_methods ! (Bucket :: key_ref) ; }
    };
}

impl_156!()