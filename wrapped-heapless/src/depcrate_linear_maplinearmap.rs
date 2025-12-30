// Generated macro for LinearMap (type)
macro_rules! Depcrate_linear_mapLinearMap {
() => {
// Module: crate::linear_map
// Provides: {"LinearMap"}
// Dependencies: {}
# [doc = " A fixed capacity map/dictionary that performs lookups via linear search."] # [doc = ""] # [doc = " Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1)."] pub type LinearMap < K , V , const N : usize > = LinearMapInner < K , V , OwnedStorage < K , V , N > > ;
};
}
