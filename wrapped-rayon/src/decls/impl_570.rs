macro_rules! deps {
    () => {
        Consumer!();
        IndexedParallelIterator!();
        ParallelIterator!();
        FoldChunks!();
    };
}

macro_rules! impl_570 {
    () => {
        deps!();
        impl < I , ID , U , F > ParallelIterator for FoldChunks < I , ID , F > where I : IndexedParallelIterator , ID : Fn () -> U + Send + Sync , F : Fn (U , I :: Item) -> U + Send + Sync , U : Send , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < U > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_570!();