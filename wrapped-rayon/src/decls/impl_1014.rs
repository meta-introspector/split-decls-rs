macro_rules! deps {
    () => {
        Zip!();
        IndexedParallelIterator!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_1014 {
    () => {
        deps!();
        impl < A , B > ParallelIterator for Zip < A , B > where A : IndexedParallelIterator , B : IndexedParallelIterator , { type Item = (A :: Item , B :: Item) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_1014!();