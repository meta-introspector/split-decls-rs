macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! BLOCK_CACHE_USAGE {
    () => {
        deps!();
        # [doc = " \"rocksdb.block-cache-usage\" - returns the memory size for the entries"] # [doc = " residing in block cache."] pub const BLOCK_CACHE_USAGE : & PropName = property ! ("block-cache-usage") ;
    };
}

BLOCK_CACHE_USAGE!()