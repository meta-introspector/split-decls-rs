macro_rules! deps {
    () => {
        Skip!();
        ParallelIterator!();
        UnindexedConsumer!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_822 {
    () => {
        deps!();
        impl < I > ParallelIterator for Skip < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_822!();