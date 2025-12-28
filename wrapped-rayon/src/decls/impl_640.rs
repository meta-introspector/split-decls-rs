macro_rules! deps {
    () => {
        InterleaveShortest!();
        ParallelIterator!();
        IndexedParallelIterator!();
        Consumer!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        impl < I , J > ParallelIterator for InterleaveShortest < I , J > where I : IndexedParallelIterator , J : IndexedParallelIterator < Item = I :: Item > , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < I :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_640!();