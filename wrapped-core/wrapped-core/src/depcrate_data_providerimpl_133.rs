// Generated macro for impl_133 (impl)
macro_rules! Depcrate_data_providerimpl_133 {
() => {
// Module: crate::data_provider
// Provides: {"impl_133"}
// Dependencies: {}
impl < M , P > DynamicDryDataProvider < M > for & P where M : DynamicDataMarker , P : DynamicDryDataProvider < M > + ? Sized , { # [inline] fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { (* self) . dry_load_data (marker , req) } }
};
}
