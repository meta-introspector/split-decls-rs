// Generated macro for impl_35 (impl)
macro_rules! Depcrate_external_loadersimpl_35 {
() => {
// Module: crate::external_loaders
// Provides: {"impl_35"}
// Dependencies: {}
impl < P > DecimalFormatterLoader for ExternalLoaderUnstable < '_ , P > where P : ? Sized + DataProvider < icu_decimal :: provider :: DecimalSymbolsV1 > + DataProvider < icu_decimal :: provider :: DecimalDigitsV1 > , { # [inline] fn load (& self , prefs : DecimalFormatterPreferences , options : DecimalFormatterOptions ,) -> Result < DecimalFormatter , DataError > { DecimalFormatter :: try_new_unstable (self . 0 , prefs , options) } }
};
}
