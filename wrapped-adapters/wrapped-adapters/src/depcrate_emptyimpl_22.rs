// Generated macro for impl_22 (impl)
macro_rules! Depcrate_emptyimpl_22 {
() => {
// Module: crate::empty
// Provides: {"impl_22"}
// Dependencies: {}
impl < M > DataProvider < M > for EmptyDataProvider where M : DataMarker , { fn load (& self , base_req : DataRequest) -> Result < DataResponse < M > , DataError > { Err (self . error_kind . with_req (M :: INFO , base_req)) } }
};
}
