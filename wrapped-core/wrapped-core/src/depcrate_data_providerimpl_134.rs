// Generated macro for impl_134 (impl)
macro_rules! Depcrate_data_providerimpl_134 {
() => {
// Module: crate::data_provider
// Provides: {"impl_134"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > DynamicDryDataProvider < M > for alloc :: boxed :: Box < P > where M : DynamicDataMarker , P : DynamicDryDataProvider < M > + ? Sized , { # [inline] fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { (* * self) . dry_load_data (marker , req) } }
};
}
