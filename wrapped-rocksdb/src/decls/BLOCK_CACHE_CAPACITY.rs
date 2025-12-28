macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! BLOCK_CACHE_CAPACITY {
    () => {
        deps!();
        # [doc = " \"rocksdb.block-cache-capacity\" - returns block cache capacity."] pub const BLOCK_CACHE_CAPACITY : & PropName = property ! ("block-cache-capacity") ;
    };
}

BLOCK_CACHE_CAPACITY!();