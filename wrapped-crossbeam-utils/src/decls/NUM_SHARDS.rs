macro_rules! NUM_SHARDS {
    () => {
        # [doc = " The number of shards per sharded lock. Must be a power of two."] const NUM_SHARDS : usize = 8 ;
    };
}

NUM_SHARDS!()