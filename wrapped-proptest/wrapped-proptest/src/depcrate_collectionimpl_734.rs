// Generated macro for impl_734 (impl)
macro_rules! Depcrate_collectionimpl_734 {
() => {
// Module: crate::collection
// Provides: {"impl_734"}
// Dependencies: {}
# [cfg (feature = "std")] impl < K : Hash + Eq , V > statics :: FilterFn < HashMap < K , V > > for MinSize { fn apply (& self , map : & HashMap < K , V >) -> bool { map . len () >= self . 0 } }
};
}
