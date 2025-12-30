// Generated macro for test_global_filters_err (function)
macro_rules! Depcrate_transliterate_compile_parsetest_global_filters_err {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_global_filters_err"}
// Dependencies: {}
# [test] fn test_global_filters_err () { let sources = [r":: [^\[$ ;" , r":: \p{L  ;" , r":: [^[$] ;" , r":: [^\[$]) ;" , r":: ( [^\[$]  ;" , r":: [^[a-z[]][]] [] ;" , r":: [^[a-z\[\]]\]] ([a-z]);" , r":: [a$-^\]] ;" , r":: ( [] [] ) ;" , r":: () [] ;" , r":: [{string}];" , r":: ([{string}]);" ,] ; for source in sources { if let Ok (rules) = parse (source) { panic ! ("Parsed invalid source {source:?}: {rules:?}") ; } } }
};
}
