macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        ParallelIterator!();
        UnindexedConsumer!();
        Rev!();
    };
}

macro_rules! impl_815 {
    () => {
        deps!();
        impl < I > ParallelIterator for Rev < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_815!()