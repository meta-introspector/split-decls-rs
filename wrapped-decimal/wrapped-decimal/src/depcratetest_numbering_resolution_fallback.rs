// Generated macro for test_numbering_resolution_fallback (function)
macro_rules! Depcratetest_numbering_resolution_fallback {
() => {
// Module: crate
// Provides: {"test_numbering_resolution_fallback"}
// Dependencies: {}
# [test] fn test_numbering_resolution_fallback () { fn test_locale (locale : icu_locale_core :: Locale , expected_format : & str) { let formatter = DecimalFormatter :: try_new ((& locale) . into () , Default :: default ()) . expect ("Must load") ; let fd = 1234 . into () ; writeable :: assert_writeable_eq ! (formatter . format (& fd) , expected_format , "Correct format for {locale}") ; } test_locale (locale ! ("en") , "1,234") ; test_locale (locale ! ("en-u-nu-arab") , "١,٢٣٤") ; test_locale (locale ! ("ar-EG") , "١٬٢٣٤") ; test_locale (locale ! ("ar-EG-u-nu-latn") , "1,234") ; test_locale (locale ! ("ar-EG-u-nu-thai") , "๑٬๒๓๔") ; test_locale (locale ! ("en-u-nu-wxyz") , "1,234") ; test_locale (locale ! ("ar-EG-u-nu-wxyz") , "١٬٢٣٤") ; }
};
}
