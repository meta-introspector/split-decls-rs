// Generated macro for impl_128 (impl)
macro_rules! Depcrate_data_providerimpl_128 {
() => {
// Module: crate::data_provider
// Provides: {"impl_128"}
// Dependencies: {}
impl < M , P > DynamicDataProvider < M > for & P where M : DynamicDataMarker , P : DynamicDataProvider < M > + ? Sized , { # [inline] fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > { (* self) . load_data (marker , req) } }
};
}
