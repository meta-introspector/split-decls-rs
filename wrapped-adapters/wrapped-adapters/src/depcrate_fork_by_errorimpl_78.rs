// Generated macro for impl_78 (impl)
macro_rules! Depcrate_fork_by_errorimpl_78 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_78"}
// Dependencies: {}
impl < M , P0 , P1 , F > IterableDynamicDataProvider < M > for ForkByErrorProvider < P0 , P1 , F > where M : DynamicDataMarker , P0 : IterableDynamicDataProvider < M > , P1 : IterableDynamicDataProvider < M > , F : ForkByErrorPredicate , { fn iter_ids_for_marker (& self , marker : DataMarkerInfo ,) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { let result = self . 0 . iter_ids_for_marker (marker) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . 2 . test (marker , None , err) => return Err (err) , _ => () , } ; self . 1 . iter_ids_for_marker (marker) } }
};
}
