// Generated macro for test_interior_nul (function)
macro_rules! Depcrate_prop_nametest_interior_nul {
() => {
// Module: crate::prop_name
// Provides: {"test_interior_nul"}
// Dependencies: {}
# [test] # [should_panic (expected = "input contained interior nul byte")] fn test_interior_nul () { PropName :: new_unwrap ("interior nul\0\0") ; }
};
}
