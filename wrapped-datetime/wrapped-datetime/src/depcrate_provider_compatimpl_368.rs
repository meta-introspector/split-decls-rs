// Generated macro for impl_368 (impl)
macro_rules! Depcrate_provider_compatimpl_368 {
() => {
// Module: crate::provider::compat
// Provides: {"impl_368"}
// Dependencies: {}
impl < M , P0 , P1 > DataProvider < M > for CompatProvider < P0 , P1 > where M : DataMarker , P0 : DataProvider < M > , P1 : BufferProvider , { fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { let original_error = match self . 0 . load (req) { Err (e @ DataError { kind : DataErrorKind :: MarkerNotFound , .. } ,) => e , other => return other , } ; if TypeId :: of :: < M > () == TypeId :: of :: < TimezonePeriodsV1 > () { crate :: provider :: time_zones :: legacy :: metazone_timezone_compat (& self . 1 , req) . and_then (DataResponse :: dynamic_cast) } else { Err (original_error) } } }
};
}
