// Generated macro for FxHashMap (type)
macro_rules! DepcrateFxHashMap {
() => {
// Module: crate
// Provides: {"FxHashMap"}
// Dependencies: {}
# [doc = " Type alias for a hash map that uses the Fx hashing algorithm."] # [cfg (feature = "std")] pub type FxHashMap < K , V > = HashMap < K , V , FxBuildHasher > ;
};
}
