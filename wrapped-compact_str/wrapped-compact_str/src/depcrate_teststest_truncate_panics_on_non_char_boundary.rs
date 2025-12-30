// Generated macro for test_truncate_panics_on_non_char_boundary (function)
macro_rules! Depcrate_teststest_truncate_panics_on_non_char_boundary {
() => {
// Module: crate::tests
// Provides: {"test_truncate_panics_on_non_char_boundary"}
// Dependencies: {}
# [test] # [should_panic (expected = "new_len must lie on char boundary")] fn test_truncate_panics_on_non_char_boundary () { let mut emojis = CompactString :: from ("😀😀😀😀") ; assert ! ('😀' . len_utf8 () > 1) ; emojis . truncate (1) ; }
};
}
