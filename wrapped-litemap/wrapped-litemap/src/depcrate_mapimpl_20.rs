// Generated macro for impl_20 (impl)
macro_rules! Depcrate_mapimpl_20 {
() => {
// Module: crate::map
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K , V > LiteMap < K , V , Vec < (K , V) > > { # [doc = " Convert a [`LiteMap`] into a sorted `Vec<(K, V)>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [inline] pub fn into_tuple_vec (self) -> Vec < (K , V) > { self . values } }
};
}
