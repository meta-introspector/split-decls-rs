macro_rules! deps {
    () => {
        UnindexedConsumer!();
        MaxLen!();
        IndexedParallelIterator!();
        ParallelIterator!();
    };
}

macro_rules! impl_669 {
    () => {
        deps!();
        impl < I > ParallelIterator for MaxLen < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_669!();