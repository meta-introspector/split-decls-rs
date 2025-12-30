// Generated macro for LinearMapView (type)
macro_rules! Depcrate_linear_mapLinearMapView {
() => {
// Module: crate::linear_map
// Provides: {"LinearMapView"}
// Dependencies: {}
# [doc = " A dynamic capacity map/dictionary that performs lookups via linear search."] # [doc = ""] # [doc = " Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1)."] pub type LinearMapView < K , V > = LinearMapInner < K , V , ViewStorage < K , V > > ;
};
}
