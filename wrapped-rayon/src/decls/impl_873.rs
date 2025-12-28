macro_rules! deps {
    () => {
        UnindexedConsumer!();
        IndexedParallelIterator!();
        Take!();
        ParallelIterator!();
    };
}

macro_rules! impl_873 {
    () => {
        deps!();
        impl < I > ParallelIterator for Take < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_873!()