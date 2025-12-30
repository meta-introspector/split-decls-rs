// Generated macro for impl_730 (impl)
macro_rules! Depcrate_collectionimpl_730 {
() => {
// Module: crate::collection
// Provides: {"impl_730"}
// Dependencies: {}
impl < T : Ord > statics :: FilterFn < BTreeSet < T > > for MinSize { fn apply (& self , set : & BTreeSet < T >) -> bool { set . len () >= self . 0 } }
};
}
