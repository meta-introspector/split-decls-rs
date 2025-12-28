macro_rules! deps {
    () => {
        UnindexedConsumer!();
        BlocksCallback!();
        UniformBlocks!();
        ParallelIterator!();
        IndexedParallelIterator!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < I > ParallelIterator for UniformBlocks < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let callback = BlocksCallback { consumer , sizes : std :: iter :: repeat (self . block_size) , len : self . base . len () , } ; self . base . with_producer (callback) } }
    };
}

impl_300!()