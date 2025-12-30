// Generated macro for tests (module)
macro_rules! Depcrate_backends_shared_posixtests {
() => {
// Module: crate::backends::shared::posix
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use icu_locale_core :: Locale ; # [test] fn test_get_raw_locale_categories () { let locale_res = raw_locale_categories () . unwrap () ; assert ! (! locale_res . is_empty () , "Empty hashmap for locales retrieved") ; for locale in locale_res . into_values () { assert ! (locale . is_ascii () , "Invalid form of locale retrieved") } } # [test] fn test_converting_locales () { let locale_res : std :: collections :: HashMap < LocaleCategory , String > = raw_locale_categories () . unwrap () ; for locale in locale_res . into_values () { let parts : Vec < & str > = locale . split ('.') . collect () ; if ! parts . contains (& "C") && (parts . len () > 1 && parts [parts . len () - 1] != "UTF-8") { let mut locale_converted : Locale = locale . parse () . unwrap () ; locale_converted . extensions . unicode . clear () ; assert_eq ! (locale_converted , locale . parse () . unwrap ()) ; } } } }
};
}
