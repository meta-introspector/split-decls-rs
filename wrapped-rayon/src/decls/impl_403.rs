macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Enumerate!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < I > ParallelIterator for Enumerate < I > where I : IndexedParallelIterator , { type Item = (usize , I :: Item) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_403!();