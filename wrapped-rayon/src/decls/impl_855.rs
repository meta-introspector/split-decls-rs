macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        StepBy!();
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_855 {
    () => {
        deps!();
        impl < I > ParallelIterator for StepBy < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_855!();