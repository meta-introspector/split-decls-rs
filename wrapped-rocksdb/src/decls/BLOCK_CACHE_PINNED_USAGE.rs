macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! BLOCK_CACHE_PINNED_USAGE {
    () => {
        deps!();
        # [doc = " \"rocksdb.block-cache-pinned-usage\" - returns the memory size for the"] # [doc = " entries being pinned."] pub const BLOCK_CACHE_PINNED_USAGE : & PropName = property ! ("block-cache-pinned-usage") ;
    };
}

BLOCK_CACHE_PINNED_USAGE!()