// Generated macro for impl_491 (impl)
macro_rules! Depcrate_preferences_localeimpl_491 {
() => {
// Module: crate::preferences::locale
// Provides: {"impl_491"}
// Dependencies: {}
impl LocalePreferences { # [doc = " Constructs a new [`LocalePreferences`] struct with the defaults."] pub const fn default () -> Self { Self { language : Language :: UNKNOWN , script : None , region : None , variant : None , subdivision : None , ue_region : None , } } # [doc = " Preference of Language"] pub const fn language (& self) -> Language { self . language } # [doc = " Preference of Region"] pub const fn region (& self) -> Option < Region > { self . region } # [doc = " Extends the preferences with the values from another set of preferences."] pub fn extend (& mut self , other : LocalePreferences) { if ! other . language . is_unknown () { self . language = other . language ; } if let Some (script) = other . script { self . script = Some (script) ; } if let Some (region) = other . region { self . region = Some (region) ; } if let Some (variant) = other . variant { self . variant = Some (variant) ; } if let Some (sd) = other . subdivision { self . subdivision = Some (sd) ; } if let Some (ue_region) = other . ue_region { self . ue_region = Some (ue_region) ; } } }
};
}
