macro_rules! deps {
    () => {
        Bucket!();
        ParIterMut!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < 'a , K : Sync + Send , V : Send > ParallelIterator for ParIterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; parallel_iterator_methods ! (Bucket :: ref_mut) ; }
    };
}

impl_113!();