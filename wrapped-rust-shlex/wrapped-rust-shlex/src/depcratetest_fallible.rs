// Generated macro for test_fallible (function)
macro_rules! Depcratetest_fallible {
() => {
// Module: crate
// Provides: {"test_fallible"}
// Dependencies: {}
# [test] fn test_fallible () { assert_eq ! (try_join (vec ! ["\0"]) , Err (QuoteError :: Nul)) ; assert_eq ! (try_quote ("\0") , Err (QuoteError :: Nul)) ; }
};
}
