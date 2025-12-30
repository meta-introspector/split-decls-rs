// Generated macro for impl_515 (impl)
macro_rules! Depcrate_shardedimpl_515 {
() => {
// Module: crate::sharded
// Provides: {"impl_515"}
// Dependencies: {}
impl < K : Eq , V > ShardedHashMap < K , V > { pub fn with_capacity (cap : usize) -> Self { Self :: new (| | HashTable :: with_capacity (cap)) } pub fn len (& self) -> usize { self . lock_shards () . map (| shard | shard . len ()) . sum () } }
};
}
