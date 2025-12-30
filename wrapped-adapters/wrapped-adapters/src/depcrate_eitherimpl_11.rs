// Generated macro for impl_11 (impl)
macro_rules! Depcrate_eitherimpl_11 {
() => {
// Module: crate::either
// Provides: {"impl_11"}
// Dependencies: {}
impl < M : DynamicDataMarker , P0 : IterableDynamicDataProvider < M > , P1 : IterableDynamicDataProvider < M > , > IterableDynamicDataProvider < M > for EitherProvider < P0 , P1 > { # [inline] fn iter_ids_for_marker (& self , marker : DataMarkerInfo ,) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { use EitherProvider :: * ; match self { A (p) => p . iter_ids_for_marker (marker) , B (p) => p . iter_ids_for_marker (marker) , } } }
};
}
