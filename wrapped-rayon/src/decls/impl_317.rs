macro_rules! deps {
    () => {
        Chunks!();
        Consumer!();
        ParallelIterator!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < I > ParallelIterator for Chunks < I > where I : IndexedParallelIterator , { type Item = Vec < I :: Item > ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < Vec < I :: Item > > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_317!();