macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        UnindexedConsumer!();
        ParallelIterator!();
        BlocksCallback!();
        ExponentialBlocks!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl < I > ParallelIterator for ExponentialBlocks < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let first = crate :: current_num_threads () ; let callback = BlocksCallback { consumer , sizes : std :: iter :: successors (Some (first) , exponential_size) , len : self . base . len () , } ; self . base . with_producer (callback) } }
    };
}

impl_296!();