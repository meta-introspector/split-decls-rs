// Generated macro for impl_89 (impl)
macro_rules! Depcrate_mapimpl_89 {
() => {
// Module: crate::map
// Provides: {"impl_89"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < K , V > IndexMap < K , V > { # [doc = " Create a new map. (Does not allocate.)"] # [inline] pub fn new () -> Self { Self :: with_capacity (0) } # [doc = " Create a new map with capacity for `n` key-value pairs. (Does not"] # [doc = " allocate if `n` is zero.)"] # [doc = ""] # [doc = " Computes in **O(n)** time."] # [inline] pub fn with_capacity (n : usize) -> Self { Self :: with_capacity_and_hasher (n , < _ > :: default ()) } }
};
}
