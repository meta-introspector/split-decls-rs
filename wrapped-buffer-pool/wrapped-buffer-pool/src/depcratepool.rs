// Generated macro for Pool (struct)
macro_rules! DepcratePool {
() => {
// Module: crate
// Provides: {"Pool"}
// Dependencies: {}
# [doc = " A sharded pool of elements."] # [derive (Debug)] pub struct Pool < const S : usize , T : 'static > { # [doc = " List of distinct shards to reduce contention."] queues : [QueueShard < T > ; S] , # [doc = " The index of the next shard to use, in round-robin order."] next_shard : AtomicUsize , }
};
}
