// Generated macro for impl_124 (impl)
macro_rules! Depcrate_data_providerimpl_124 {
() => {
// Module: crate::data_provider
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > DryDataProvider < M > for alloc :: rc :: Rc < P > where M : DataMarker , P : DryDataProvider < M > + ? Sized , { # [inline] fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { (* * self) . dry_load (req) } }
};
}
