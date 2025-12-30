// Generated macro for FxHashSet (type)
macro_rules! DepcrateFxHashSet {
() => {
// Module: crate
// Provides: {"FxHashSet"}
// Dependencies: {}
# [doc = " Type alias for a hash set that uses the Fx hashing algorithm."] # [cfg (feature = "std")] pub type FxHashSet < V > = HashSet < V , FxBuildHasher > ;
};
}
