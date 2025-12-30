// Generated macro for matches_pattern_impl (function)
macro_rules! Depcrate_matches_patternmatches_pattern_impl {
() => {
// Module: crate::matches_pattern
// Provides: {"matches_pattern_impl"}
// Dependencies: {}
# [doc = " This is an implementation detail of `googletest::matches_pattern!`. It"] # [doc = " assumes that a few symbols from `googletest::matchers` have been imported"] # [doc = " and that `$crate` has been aliased to `googletest` (which might otherwise"] # [doc = " have been imported as a different alias), both of which are done"] # [doc = " by `googletest::matches_pattern!` before calling this proc macro."] pub (crate) fn matches_pattern_impl (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let result = parse_macro_input ! (input as ParsedMatchPattern) . into_matcher_expr () ; quote ! { # result } . into () }
};
}
