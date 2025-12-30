// Generated macro for icu_locale_from_unic_langid (function)
macro_rules! Depcrateicu_locale_from_unic_langid {
() => {
// Module: crate
// Provides: {"icu_locale_from_unic_langid"}
// Dependencies: {}
fn icu_locale_from_unic_langid (lang : LanguageIdentifier) -> Option < icu_locale :: Locale > { icu_locale :: Locale :: try_from_str (& lang . to_string ()) . ok () }
};
}
