// Generated macro for impl_82 (impl)
macro_rules! Depcrate_fork_by_errorimpl_82 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_82"}
// Dependencies: {}
impl < M , P , F > DataProvider < M > for MultiForkByErrorProvider < P , F > where M : DataMarker , P : DataProvider < M > , F : ForkByErrorPredicate , { fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { let mut last_error = F :: UNIT_ERROR . with_marker (M :: INFO) ; for provider in self . providers . iter () { let result = provider . load (req) ; match result { Ok (ok) => return Ok (ok) , Err (err) if ! self . predicate . test (M :: INFO , Some (req) , err) => return Err (err) , Err (err) => last_error = err , } ; } Err (last_error) } }
};
}
