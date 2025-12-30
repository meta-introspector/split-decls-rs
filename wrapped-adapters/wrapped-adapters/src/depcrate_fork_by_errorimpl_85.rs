// Generated macro for impl_85 (impl)
macro_rules! Depcrate_fork_by_errorimpl_85 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_85"}
// Dependencies: {}
impl < M , P , F > DynamicDryDataProvider < M > for MultiForkByErrorProvider < P , F > where M : DynamicDataMarker , P : DynamicDryDataProvider < M > , F : ForkByErrorPredicate , { fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { let mut last_error = F :: UNIT_ERROR . with_marker (marker) ; for provider in self . providers . iter () { let result = provider . dry_load_data (marker , req) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . predicate . test (marker , Some (req) , err) => return Err (err) , Err (err) => last_error = err , } ; } Err (last_error) } }
};
}
