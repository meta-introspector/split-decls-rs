// Generated macro for impl_140 (impl)
macro_rules! Depcrate_shortvecimpl_140 {
() => {
// Module: crate::shortvec
// Provides: {"impl_140"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > FromIterator < T > for ShortBoxSlice < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { use ShortBoxSliceInner :: * ; let mut iter = iter . into_iter () ; match (iter . next () , iter . next ()) { (Some (first) , Some (second)) => { let mut vec = Vec :: with_capacity (iter . size_hint () . 0 . saturating_add (3)) ; vec . push (first) ; vec . push (second) ; vec . extend (iter) ; Self (Multi (vec . into_boxed_slice ())) } (first , _) => Self (ZeroOne (first)) , } } }
};
}
