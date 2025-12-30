// Generated macro for tests (module)
macro_rules! Depcrate_extensions_unicode_keywordstests {
() => {
// Module: crate::extensions::unicode::keywords
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_keywords_fromstr () { let kw : Keywords = "hc-h12" . parse () . expect ("Failed to parse Keywords") ; assert_eq ! (kw . to_string () , "hc-h12") ; } }
};
}
