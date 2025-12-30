// Generated macro for impl_738 (impl)
macro_rules! Depcrate_collectionimpl_738 {
() => {
// Module: crate::collection
// Provides: {"impl_738"}
// Dependencies: {}
impl < K : Ord , V > statics :: FilterFn < BTreeMap < K , V > > for MinSize { fn apply (& self , map : & BTreeMap < K , V >) -> bool { map . len () >= self . 0 } }
};
}
