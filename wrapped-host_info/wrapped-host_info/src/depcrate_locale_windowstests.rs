// Generated macro for tests (module)
macro_rules! Depcrate_locale_windowstests {
() => {
// Module: crate::locale::windows
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; fn expect_success (src : & str , expected : & str) { let windows_locale = WindowsLocale :: try_from_str (src) . expect (src) ; let locale = Locale :: try_from (windows_locale) . expect (src) ; assert_eq ! (locale , Locale :: try_from_str (expected) . unwrap () , "Case: {src}") ; } # [test] fn collation () { # [doc = " All MS-LCID collation entries with a known matching CLDR collation value"] const CASES : [(& str , & str) ; 12] = [("de-DE_phoneb" , "de-DE-u-co-phonebk") , ("es-ES_tradnl" , "es-ES-u-co-trad") , ("ja-JP_radstr" , "ja-JP-u-co-unihan") , ("zh-CN_phoneb" , "zh-CN-u-co-phonebk") , ("zh-CN_stroke" , "zh-CN-u-co-stroke") , ("zh-HK_radstr" , "zh-HK-u-co-unihan") , ("zh-MO_radstr" , "zh-MO-u-co-unihan") , ("zh-MO_stroke" , "zh-MO-u-co-stroke") , ("zh-SG_phoneb" , "zh-SG-u-co-phonebk") , ("zh-SG_stroke" , "zh-SG-u-co-stroke") , ("zh-TW_pronun" , "zh-TW-u-co-zhuyin") , ("zh-TW_radstr" , "zh-TW-u-co-unihan") ,] ; for (src , expected) in CASES { expect_success (src , expected) ; } } # [test] fn collation_strip_known_invalid () { expect_success ("hu-HU_tchncl" , "hu-HU") ; expect_success ("ka-GE_modern" , "ka-GE") ; } # [test] fn collation_strip_unknown () { expect_success ("en-US_unknown" , "en-US") ; expect_success ("en-US_unknown_multiple_underscores" , "en-US") ; expect_success ("en-US_unknown-with-hyphens" , "en-US") ; } # [test] fn alias () { expect_success ("zh-yue-HK" , "yue-HK") ; expect_success ("x-IV-mathan" , "und") ; } }
};
}
