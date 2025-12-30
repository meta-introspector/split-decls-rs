// Generated macro for test_transform_rules_err (function)
macro_rules! Depcrate_transliterate_compile_parsetest_transform_rules_err {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_transform_rules_err"}
// Dependencies: {}
# [test] fn test_transform_rules_err () { let sources = [r":: a a ;" , r":: (a a) ;" , r":: a - z - b ;" , r":: ( a - z - b) ;" , r":: [] ( a - z) ;" , r":: a-z ( [] ) ;" , r":: a-z / ( [] a-z ) ;" , r":: Latin-ASCII/BGN Arab-Greek/UNGEGN ;" , r":: (Latin-ASCII/BGN Arab-Greek/UNGEGN) ;" , r":: [a-z{string}] Remove ;" ,] ; for source in sources { if let Ok (rules) = parse (source) { panic ! ("Parsed invalid source {source:?}: {rules:?}") ; } } }
};
}
