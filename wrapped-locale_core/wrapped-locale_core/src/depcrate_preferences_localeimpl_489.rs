// Generated macro for impl_489 (impl)
macro_rules! Depcrate_preferences_localeimpl_489 {
() => {
// Module: crate::preferences::locale
// Provides: {"impl_489"}
// Dependencies: {}
impl From < & crate :: LanguageIdentifier > for LocalePreferences { fn from (lid : & crate :: LanguageIdentifier) -> Self { Self { language : lid . language , script : lid . script , region : lid . region , variant : lid . variants . iter () . copied () . next () , subdivision : None , ue_region : None , } } }
};
}
