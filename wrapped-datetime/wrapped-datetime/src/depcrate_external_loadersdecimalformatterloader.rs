// Generated macro for DecimalFormatterLoader (trait)
macro_rules! Depcrate_external_loadersDecimalFormatterLoader {
() => {
// Module: crate::external_loaders
// Provides: {"DecimalFormatterLoader"}
// Dependencies: {}
# [doc = " Trait for loading a DecimalFormatter."] # [doc = ""] # [doc = " Implemented on the provider-specific loader types in this module."] pub (crate) trait DecimalFormatterLoader { fn load (& self , prefs : DecimalFormatterPreferences , options : DecimalFormatterOptions ,) -> Result < DecimalFormatter , DataError > ; }
};
}
