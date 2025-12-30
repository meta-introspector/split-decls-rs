// Generated macro for impl_131 (impl)
macro_rules! Depcrate_data_providerimpl_131 {
() => {
// Module: crate::data_provider
// Provides: {"impl_131"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [cfg (feature = "alloc")] impl < M , P > DynamicDataProvider < M > for alloc :: sync :: Arc < P > where M : DynamicDataMarker , P : DynamicDataProvider < M > + ? Sized , { # [inline] fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > { (* * self) . load_data (marker , req) } }
};
}
