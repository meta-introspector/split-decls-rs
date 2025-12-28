macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        MinLen!();
        ParallelIterator!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_663 {
    () => {
        deps!();
        impl < I > ParallelIterator for MinLen < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_663!();