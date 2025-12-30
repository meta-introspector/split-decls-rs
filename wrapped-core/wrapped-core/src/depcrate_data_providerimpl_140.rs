// Generated macro for impl_140 (impl)
macro_rules! Depcrate_data_providerimpl_140 {
() => {
// Module: crate::data_provider
// Provides: {"impl_140"}
// Dependencies: {}
impl < M , P > BoundDataProvider < M > for & P where M : DynamicDataMarker , P : BoundDataProvider < M > + ? Sized , { # [inline] fn load_bound (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { (* self) . load_bound (req) } # [inline] fn bound_marker (& self) -> DataMarkerInfo { (* self) . bound_marker () } }
};
}
