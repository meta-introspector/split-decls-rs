macro_rules! deps {
    () => {
        ParDrain!();
        Bucket!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < K : Send , V : Send > ParallelIterator for ParDrain < '_ , K , V > { type Item = (K , V) ; parallel_iterator_methods ! (Bucket :: key_value) ; }
    };
}

impl_117!();