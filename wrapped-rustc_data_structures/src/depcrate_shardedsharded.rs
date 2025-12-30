// Generated macro for Sharded (enum)
macro_rules! Depcrate_shardedSharded {
() => {
// Module: crate::sharded
// Provides: {"Sharded"}
// Dependencies: {}
# [doc = " An array of cache-line aligned inner locked structures with convenience methods."] # [doc = " A single field is used when the compiler uses only one thread."] pub enum Sharded < T > { Single (Lock < T >) , Shards (Box < [CacheAligned < Lock < T > > ; SHARDS] >) , }
};
}
