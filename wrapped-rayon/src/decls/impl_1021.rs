macro_rules! deps {
    () => {
        ParallelIterator!();
        ZipEq!();
        IndexedParallelIterator!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_1021 {
    () => {
        deps!();
        impl < A , B > ParallelIterator for ZipEq < A , B > where A : IndexedParallelIterator , B : IndexedParallelIterator , { type Item = (A :: Item , B :: Item) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self . zip , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . zip . len ()) } }
    };
}

impl_1021!();