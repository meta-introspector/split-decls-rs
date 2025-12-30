// Generated macro for make_hash (function)
macro_rules! Depcrate_shardedmake_hash {
() => {
// Module: crate::sharded
// Provides: {"make_hash"}
// Dependencies: {}
# [inline] pub fn make_hash < K : Hash + ? Sized > (val : & K) -> u64 { let mut state = FxHasher :: default () ; val . hash (& mut state) ; state . finish () }
};
}
