// Generated macro for test_conversion_rules_ok (function)
macro_rules! Depcrate_transliterate_compile_parsetest_conversion_rules_ok {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_conversion_rules_ok"}
// Dependencies: {}
# [test] fn test_conversion_rules_ok () { let sources = [r"a > b ;" , r"a < b ;" , r"a <> b ;" , r"a → b ;" , r"a ← b ;" , r"a ↔ b ;" , r"a \> > b ;" , r"a \→ > b ;" , r"{ a > b ;" , r"a {  > b ;" , r"{ a } > b ;" , r"{ a } > { b ;" , r"{ a } > { b } ;" , r"^ pre [a-z] { a } post [$] $ > ^ [$] pre { b [b-z] } post $ ;" , r"[äöü] > ;" , r"([äöü]) > &Remove($1) ;" , r"[äöü] { ([äöü]+) > &Remove($1) ;" , r"|@@@ a <> b @@@@  @ | ;" , r"|a <> b ;" ,] ; for source in sources { parse (source) . map_err (| e | e . explain (source)) . unwrap () ; } }
};
}
