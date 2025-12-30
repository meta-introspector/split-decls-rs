// Generated macro for impl_123 (impl)
macro_rules! Depcrate_data_providerimpl_123 {
() => {
// Module: crate::data_provider
// Provides: {"impl_123"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > DryDataProvider < M > for alloc :: boxed :: Box < P > where M : DataMarker , P : DryDataProvider < M > + ? Sized , { # [inline] fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { (* * self) . dry_load (req) } }
};
}
