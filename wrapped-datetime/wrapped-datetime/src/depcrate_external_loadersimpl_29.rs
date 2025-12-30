// Generated macro for impl_29 (impl)
macro_rules! Depcrate_external_loadersimpl_29 {
() => {
// Module: crate::external_loaders
// Provides: {"impl_29"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] impl DecimalFormatterLoader for ExternalLoaderCompiledData { # [inline] fn load (& self , prefs : DecimalFormatterPreferences , options : DecimalFormatterOptions ,) -> Result < DecimalFormatter , DataError > { DecimalFormatter :: try_new (prefs , options) } }
};
}
