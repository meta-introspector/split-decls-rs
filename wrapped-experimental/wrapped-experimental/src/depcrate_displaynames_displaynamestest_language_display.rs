// Generated macro for test_language_display (function)
macro_rules! Depcrate_displaynames_displaynamestest_language_display {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"test_language_display"}
// Dependencies: {}
# [test] fn test_language_display () { use icu_locale_core :: locale ; let dialect = LocaleDisplayNamesFormatter :: try_new (locale ! ("en") . into () , DisplayNamesOptions { language_display : LanguageDisplay :: Dialect , .. Default :: default () } ,) . unwrap () ; let standard = LocaleDisplayNamesFormatter :: try_new (locale ! ("en") . into () , DisplayNamesOptions { language_display : LanguageDisplay :: Standard , .. Default :: default () } ,) . unwrap () ; assert_eq ! (dialect . of (& locale ! ("en-GB")) , "British English") ; assert_eq ! (standard . of (& locale ! ("en-GB")) , "English (United Kingdom)") ; assert_eq ! (dialect . of (& locale ! ("zh-Hant")) , "Traditional Chinese") ; assert_eq ! (standard . of (& locale ! ("zh-Hant")) , "Chinese (Traditional)") ; }
};
}
