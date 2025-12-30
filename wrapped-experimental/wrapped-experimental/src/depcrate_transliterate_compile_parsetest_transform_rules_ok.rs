// Generated macro for test_transform_rules_ok (function)
macro_rules! Depcrate_transliterate_compile_parsetest_transform_rules_ok {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_transform_rules_ok"}
// Dependencies: {}
# [test] fn test_transform_rules_ok () { let sources = [":: NFD; :: NFKC;" , ":: Latin ;" , ":: any - Latin;" , ":: any - Latin/bgn;" , ":: any - Latin/bgn ();" , ":: any - Latin/bgn ([a-z] a-z);" , ":: ([a-z] a-z);" , ":: (a-z);" , ":: (a-z / variant);" , ":: [a-z] latin/variant (a-z / variant);" , ":: [a-z] latin/variant (a-z / variant) ;" , ":: [a-z] latin (  );" , ":: [a-z] latin ;" , "::[];" ,] ; for source in sources { parse (source) . map_err (| e | e . explain (source)) . unwrap () ; } }
};
}
