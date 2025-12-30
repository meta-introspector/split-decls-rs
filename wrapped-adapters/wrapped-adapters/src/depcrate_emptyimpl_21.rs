// Generated macro for impl_21 (impl)
macro_rules! Depcrate_emptyimpl_21 {
() => {
// Module: crate::empty
// Provides: {"impl_21"}
// Dependencies: {}
impl < M > DynamicDataProvider < M > for EmptyDataProvider where M : DynamicDataMarker , { fn load_data (& self , marker : DataMarkerInfo , base_req : DataRequest ,) -> Result < DataResponse < M > , DataError > { Err (self . error_kind . with_req (marker , base_req)) } }
};
}
