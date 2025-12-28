macro_rules! deps {
    () => {
        ParallelIterator!();
        Consumer!();
        IndexedParallelIterator!();
        FoldChunksWith!();
    };
}

macro_rules! impl_577 {
    () => {
        deps!();
        impl < I , U , F > ParallelIterator for FoldChunksWith < I , U , F > where I : IndexedParallelIterator , U : Send + Clone , F : Fn (U , I :: Item) -> U + Send + Sync , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : Consumer < U > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_577!();