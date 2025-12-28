macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        ParallelIterator!();
        Consumer!();
        Interleave!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl < I , J > ParallelIterator for Interleave < I , J > where I : IndexedParallelIterator , J : IndexedParallelIterator < Item = I :: Item > , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < I :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_628!();