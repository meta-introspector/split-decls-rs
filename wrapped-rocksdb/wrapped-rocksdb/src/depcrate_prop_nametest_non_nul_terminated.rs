// Generated macro for test_non_nul_terminated (function)
macro_rules! Depcrate_prop_nametest_non_nul_terminated {
() => {
// Module: crate::prop_name
// Provides: {"test_non_nul_terminated"}
// Dependencies: {}
# [test] # [should_panic (expected = "input was not nul-terminated")] fn test_non_nul_terminated () { PropName :: new_unwrap ("no nul terminator") ; }
};
}
