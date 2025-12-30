// Generated macro for impl_2926 (impl)
macro_rules! Depcrate_ctxhashimpl_2926 {
() => {
// Module: crate::ctxhash
// Provides: {"impl_2926"}
// Dependencies: {}
impl < K , V > CtxHashMap < K , V > { # [doc = " Create an empty hashmap with pre-allocated space for the given"] # [doc = " capacity."] pub fn with_capacity (capacity : usize) -> Self { Self { raw : HashTable :: with_capacity (capacity) , } } }
};
}
