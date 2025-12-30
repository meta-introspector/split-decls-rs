// Generated macro for tests (module)
macro_rules! Depcrate_backends_macostests {
() => {
// Module: crate::backends::macos
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: backends :: { macos :: MacOSHostInfoBackend , RawHostInfoBackend } ; use icu_locale_core :: Locale ; # [test] fn test_get_raw_locales () { let locales_res = MacOSHostInfoBackend :: raw_requested_locales () ; match locales_res { Ok (locales) => { for locale in locales { assert ! (! locale . is_empty () , "Empty locale retrieved") ; assert ! (locale . is_ascii () , "Invalid form of locale retrieved") ; } } Err (e) => { panic ! ("{e:?}") } } } # [test] fn test_converting_locales () { let locales = MacOSHostInfoBackend :: raw_requested_locales () . unwrap () ; for locale in locales { let _loc : Locale = locale . parse () . unwrap () ; } } # [test] fn test_calendar () { let calendar = MacOSHostInfoBackend :: raw_calendar () . unwrap () ; assert ! (calendar . is_some () , "Couldn't retrieve calendar") ; assert ! (calendar . unwrap () . is_ascii () , "Calendar identifier form is not valid") ; } }
};
}
