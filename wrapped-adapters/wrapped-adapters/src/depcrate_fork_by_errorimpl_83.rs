// Generated macro for impl_83 (impl)
macro_rules! Depcrate_fork_by_errorimpl_83 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_83"}
// Dependencies: {}
impl < M , P , F > DryDataProvider < M > for MultiForkByErrorProvider < P , F > where M : DataMarker , P : DryDataProvider < M > , F : ForkByErrorPredicate , { fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { let mut last_error = F :: UNIT_ERROR . with_marker (M :: INFO) ; for provider in self . providers . iter () { let result = provider . dry_load (req) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . predicate . test (M :: INFO , Some (req) , err) => return Err (err) , Err (err) => last_error = err , } ; } Err (last_error) } }
};
}
