// Generated macro for LocalePreferences (struct)
macro_rules! Depcrate_preferences_localeLocalePreferences {
() => {
// Module: crate::preferences::locale
// Provides: {"LocalePreferences"}
// Dependencies: {}
# [doc = " The structure storing locale subtags used in preferences."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct LocalePreferences { # [doc = " Preference of Language"] pub (crate) language : Language , # [doc = " Preference of Script"] pub (crate) script : Option < Script > , # [doc = " Preference of Region"] pub (crate) region : Option < Region > , # [doc = " Preference of Variant"] pub (crate) variant : Option < Variant > , # [doc = " Preference of Regional Subdivision"] pub (crate) subdivision : Option < Subtag > , # [doc = " Preference of Unicode Extension Region"] pub (crate) ue_region : Option < Region > , }
};
}
