// Generated macro for impl_285 (impl)
macro_rules! Depcrate_pattern_namesimpl_285 {
() => {
// Module: crate::pattern::names
// Provides: {"impl_285"}
// Dependencies: {}
impl < M > DataProvider < M > for EmptyDataProvider where M : DataMarker , { fn load (& self , base_req : DataRequest) -> Result < DataResponse < M > , DataError > { Err (DataErrorKind :: MarkerNotFound . with_req (M :: INFO , base_req)) } }
};
}
