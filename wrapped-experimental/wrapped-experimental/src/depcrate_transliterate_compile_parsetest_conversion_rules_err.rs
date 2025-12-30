// Generated macro for test_conversion_rules_err (function)
macro_rules! Depcrate_transliterate_compile_parsetest_conversion_rules_err {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_conversion_rules_err"}
// Dependencies: {}
# [test] fn test_conversion_rules_err () { let sources = [r"a > > b ;" , r"a >< b ;" , r"(a > b) > b ;" , r"a \← b ;" , r"a ↔ { b > } ;" , r"a ↔ { b > } ;" , r"a > b" , r"@ a > b ;" , r"a ( {  > b ;" , r"a ( { )  > b ;" , r"a } + > b ;" , r"a (+?*) > b ;" , r"+?* > b ;" , r"+ > b ;" , r"* > b ;" , r"? > b ;" , r"use variable range 0x71 > 2 ; use variable range 0x71 ;" ,] ; for source in sources { parse (source) . unwrap_err () ; } }
};
}
