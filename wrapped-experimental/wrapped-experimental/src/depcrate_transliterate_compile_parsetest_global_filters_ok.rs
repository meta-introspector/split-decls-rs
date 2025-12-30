// Generated macro for test_global_filters_ok (function)
macro_rules! Depcrate_transliterate_compile_parsetest_global_filters_ok {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_global_filters_ok"}
// Dependencies: {}
# [test] fn test_global_filters_ok () { let sources = [r":: [^\[$] ;" , r":: \p{L} ;" , r":: [^\[{[}$] ;" , r":: [^\[{]}$] ;" , r":: [^\[{]\}]}$] ;" , r":: ([^\[$]) ;" , r":: ( [^\[$] ) ;" , r":: [^[a-z[]][]] ;" , r":: [^[a-z\[\]]\]] ;" , r":: [^\]] ;" ,] ; for source in sources { parse (source) . map_err (| e | e . explain (source)) . unwrap () ; } }
};
}
