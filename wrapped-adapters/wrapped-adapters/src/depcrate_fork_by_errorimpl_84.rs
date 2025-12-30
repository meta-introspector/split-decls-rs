// Generated macro for impl_84 (impl)
macro_rules! Depcrate_fork_by_errorimpl_84 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_84"}
// Dependencies: {}
impl < M , P , F > DynamicDataProvider < M > for MultiForkByErrorProvider < P , F > where M : DynamicDataMarker , P : DynamicDataProvider < M > , F : ForkByErrorPredicate , { fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > { let mut last_error = F :: UNIT_ERROR . with_marker (marker) ; for provider in self . providers . iter () { let result = provider . load_data (marker , req) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . predicate . test (marker , Some (req) , err) => return Err (err) , Err (err) => last_error = err , } ; } Err (last_error) } }
};
}
