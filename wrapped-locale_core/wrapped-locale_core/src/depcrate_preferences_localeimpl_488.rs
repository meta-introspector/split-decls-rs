// Generated macro for impl_488 (impl)
macro_rules! Depcrate_preferences_localeimpl_488 {
() => {
// Module: crate::preferences::locale
// Provides: {"impl_488"}
// Dependencies: {}
impl From < & crate :: Locale > for LocalePreferences { fn from (loc : & crate :: Locale) -> Self { let sd = loc . extensions . unicode . keywords . get (& crate :: extensions :: unicode :: key ! ("sd")) . and_then (| v | v . as_single_subtag () . copied ()) ; let ue_region = loc . extensions . unicode . keywords . get (& crate :: extensions :: unicode :: key ! ("rg")) . and_then (| v | { v . as_single_subtag () . and_then (| s | Region :: try_from_str (s . as_str ()) . ok ()) }) ; Self { language : loc . id . language , script : loc . id . script , region : loc . id . region , variant : loc . id . variants . iter () . copied () . next () , subdivision : sd , ue_region , } } }
};
}
