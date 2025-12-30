// Generated macro for impl_726 (impl)
macro_rules! Depcrate_collectionimpl_726 {
() => {
// Module: crate::collection
// Provides: {"impl_726"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : Eq + Hash > statics :: FilterFn < HashSet < T > > for MinSize { fn apply (& self , set : & HashSet < T >) -> bool { set . len () >= self . 0 } }
};
}
