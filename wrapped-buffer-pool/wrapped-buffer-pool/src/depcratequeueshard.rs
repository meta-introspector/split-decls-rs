// Generated macro for QueueShard (struct)
macro_rules! DepcrateQueueShard {
() => {
// Module: crate
// Provides: {"QueueShard"}
// Dependencies: {}
# [derive (Debug)] struct QueueShard < T > { # [doc = " The inner stack of pooled values."] queue : SegQueue < T > , # [doc = " The number of elements currently stored in this shard."] elem_cnt : AtomicUsize , # [doc = " The value to use when calling [`Reuse::reuse`]. Typically the capacity"] # [doc = " to keep in a reused buffer."] trim : usize , # [doc = " The max number of values to keep in the shard."] max : usize , }
};
}
