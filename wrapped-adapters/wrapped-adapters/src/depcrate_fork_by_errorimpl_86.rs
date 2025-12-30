// Generated macro for impl_86 (impl)
macro_rules! Depcrate_fork_by_errorimpl_86 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_86"}
// Dependencies: {}
impl < M , P , F > IterableDynamicDataProvider < M > for MultiForkByErrorProvider < P , F > where M : DynamicDataMarker , P : IterableDynamicDataProvider < M > , F : ForkByErrorPredicate , { fn iter_ids_for_marker (& self , marker : DataMarkerInfo ,) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { let mut last_error = F :: UNIT_ERROR . with_marker (marker) ; for provider in self . providers . iter () { let result = provider . iter_ids_for_marker (marker) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . predicate . test (marker , None , err) => return Err (err) , Err (err) => last_error = err , } ; } Err (last_error) } }
};
}
