// Generated macro for impl_486 (impl)
macro_rules! Depcrate_preferences_localeimpl_486 {
() => {
// Module: crate::preferences::locale
// Provides: {"impl_486"}
// Dependencies: {}
impl LocalePreferences { fn to_data_locale_maybe_region_priority (self , region_priority : bool) -> DataLocale { DataLocale { language : self . language , script : self . script , region : match (self . region , self . ue_region) { (Some (_) , Some (r)) if region_priority => Some (r) , (r , _) => r , } , variant : self . variant , subdivision : self . subdivision , } } # [doc = " Convert to a DataLocale, with region-based fallback priority"] # [doc = ""] # [doc = " Most users should use `icu_provider::marker::make_locale()` instead."] pub fn to_data_locale_region_priority (self) -> DataLocale { self . to_data_locale_maybe_region_priority (true) } # [doc = " Convert to a DataLocale, with language-based fallback priority"] # [doc = ""] # [doc = " Most users should use `icu_provider::marker::make_locale()` instead."] pub fn to_data_locale_language_priority (self) -> DataLocale { self . to_data_locale_maybe_region_priority (false) } }
};
}
