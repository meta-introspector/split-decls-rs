// Generated macro for impl_12 (impl)
macro_rules! Depcrate_eitherimpl_12 {
() => {
// Module: crate::either
// Provides: {"impl_12"}
// Dependencies: {}
impl < M : DataMarker , P0 : IterableDataProvider < M > , P1 : IterableDataProvider < M > > IterableDataProvider < M > for EitherProvider < P0 , P1 > { # [inline] fn iter_ids (& self) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { use EitherProvider :: * ; match self { A (p) => p . iter_ids () , B (p) => p . iter_ids () , } } }
};
}
