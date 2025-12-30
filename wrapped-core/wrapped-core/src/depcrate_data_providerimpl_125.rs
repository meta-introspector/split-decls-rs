// Generated macro for impl_125 (impl)
macro_rules! Depcrate_data_providerimpl_125 {
() => {
// Module: crate::data_provider
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [cfg (feature = "alloc")] impl < M , P > DryDataProvider < M > for alloc :: sync :: Arc < P > where M : DataMarker , P : DryDataProvider < M > + ? Sized , { # [inline] fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { (* * self) . dry_load (req) } }
};
}
