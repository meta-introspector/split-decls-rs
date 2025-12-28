macro_rules! deps {
    () => {
        Cache!();
        LruCacheOptions!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl LruCacheOptions { # [doc = " Capacity of the cache, in the same units as the `charge` of each entry."] # [doc = " This is typically measured in bytes, but can be a different unit if using"] # [doc = " kDontChargeCacheMetadata."] pub fn set_capacity (& mut self , cap : usize) { unsafe { ffi :: rocksdb_lru_cache_options_set_capacity (self . inner , cap) ; } } # [doc = " Cache is sharded into 2^num_shard_bits shards, by hash of key."] # [doc = " If < 0, a good default is chosen based on the capacity and the"] # [doc = " implementation. (Mutex-based implementations are much more reliant"] # [doc = " on many shards for parallel scalability.)"] pub fn set_num_shard_bits (& mut self , val : c_int) { unsafe { ffi :: rocksdb_lru_cache_options_set_num_shard_bits (self . inner , val) ; } } }
    };
}

impl_205!()