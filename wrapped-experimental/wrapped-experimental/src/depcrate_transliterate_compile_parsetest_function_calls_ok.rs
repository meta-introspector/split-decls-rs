// Generated macro for test_function_calls_ok (function)
macro_rules! Depcrate_transliterate_compile_parsetest_function_calls_ok {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_function_calls_ok"}
// Dependencies: {}
# [test] fn test_function_calls_ok () { let sources = [r"$fn = & Any-Any/Variant ($var literal 'quoted literal' $1) ;" , r"$fn = &[a-z] Any-Any/Variant ($var literal 'quoted literal' $1) ;" , r"$fn = &[a-z]Any-Any/Variant ($var literal 'quoted literal' $1) ;" , r"$fn = &[a-z]Any/Variant ($var literal 'quoted literal' $1) ;" , r"$fn = &Any/Variant ($var literal 'quoted literal' $1) ;" , r"$fn = &[a-z]Any ($var literal 'quoted literal' $1) ;" , r"$fn = &Any($var literal 'quoted literal' $1) ;" ,] ; for source in sources { parse (source) . map_err (| e | e . explain (source)) . unwrap () ; } }
};
}
