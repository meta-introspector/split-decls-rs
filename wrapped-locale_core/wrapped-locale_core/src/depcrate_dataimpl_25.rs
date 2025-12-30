// Generated macro for impl_25 (impl)
macro_rules! Depcrate_dataimpl_25 {
() => {
// Module: crate::data
// Provides: {"impl_25"}
// Dependencies: {}
impl From < & LanguageIdentifier > for DataLocale { fn from (langid : & LanguageIdentifier) -> Self { Self { language : langid . language , script : langid . script , region : langid . region , variant : langid . variants . iter () . copied () . next () , subdivision : None , } } }
};
}
