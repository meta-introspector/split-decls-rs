// Generated macro for impl_135 (impl)
macro_rules! Depcrate_data_providerimpl_135 {
() => {
// Module: crate::data_provider
// Provides: {"impl_135"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > DynamicDryDataProvider < M > for alloc :: rc :: Rc < P > where M : DynamicDataMarker , P : DynamicDryDataProvider < M > + ? Sized , { # [inline] fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { (* * self) . dry_load_data (marker , req) } }
};
}
