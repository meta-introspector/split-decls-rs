macro_rules! deps {
    () => {
        ParValuesMut!();
        Bucket!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'a , K : Send , V : Send > ParallelIterator for ParValuesMut < 'a , K , V > { type Item = & 'a mut V ; parallel_iterator_methods ! (Bucket :: value_mut) ; }
    };
}

impl_137!()