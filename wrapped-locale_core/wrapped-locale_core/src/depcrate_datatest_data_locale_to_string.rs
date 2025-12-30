// Generated macro for test_data_locale_to_string (function)
macro_rules! Depcrate_datatest_data_locale_to_string {
() => {
// Module: crate::data
// Provides: {"test_data_locale_to_string"}
// Dependencies: {}
# [test] fn test_data_locale_to_string () { struct TestCase { pub locale : & 'static str , pub expected : & 'static str , } for cas in [TestCase { locale : "und" , expected : "und" , } , TestCase { locale : "und-u-sd-sdd" , expected : "und-u-sd-sdd" , } , TestCase { locale : "en-ZA-u-sd-zaa" , expected : "en-ZA-u-sd-zaa" , } ,] { let locale = cas . locale . parse :: < DataLocale > () . unwrap () ; writeable :: assert_writeable_eq ! (locale , cas . expected) ; } }
};
}
