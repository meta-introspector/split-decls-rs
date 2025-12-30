// Generated macro for impl_122 (impl)
macro_rules! Depcrate_data_providerimpl_122 {
() => {
// Module: crate::data_provider
// Provides: {"impl_122"}
// Dependencies: {}
impl < M , P > DryDataProvider < M > for & P where M : DataMarker , P : DryDataProvider < M > + ? Sized , { # [inline] fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { (* self) . dry_load (req) } }
};
}
