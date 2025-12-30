// Generated macro for FxHashMap (type)
macro_rules! Depcrate_bridge_fxhashFxHashMap {
() => {
// Module: crate::bridge::fxhash
// Provides: {"FxHashMap"}
// Dependencies: {}
# [doc = " Type alias for a hashmap using the `fx` hash algorithm."] pub (super) type FxHashMap < K , V > = HashMap < K , V , BuildHasherDefault < FxHasher > > ;
};
}
