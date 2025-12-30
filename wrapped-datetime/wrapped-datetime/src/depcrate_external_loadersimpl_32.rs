// Generated macro for impl_32 (impl)
macro_rules! Depcrate_external_loadersimpl_32 {
() => {
// Module: crate::external_loaders
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < P > DecimalFormatterLoader for ExternalLoaderBuffer < '_ , P > where P : ? Sized + BufferProvider , { # [inline] fn load (& self , prefs : DecimalFormatterPreferences , options : DecimalFormatterOptions ,) -> Result < DecimalFormatter , DataError > { DecimalFormatter :: try_new_with_buffer_provider (self . 0 , prefs , options) } }
};
}
