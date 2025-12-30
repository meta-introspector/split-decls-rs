// Generated macro for impl_136 (impl)
macro_rules! Depcrate_data_providerimpl_136 {
() => {
// Module: crate::data_provider
// Provides: {"impl_136"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [cfg (feature = "alloc")] impl < M , P > DynamicDryDataProvider < M > for alloc :: sync :: Arc < P > where M : DynamicDataMarker , P : DynamicDryDataProvider < M > + ? Sized , { # [inline] fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { (* * self) . dry_load_data (marker , req) } }
};
}
