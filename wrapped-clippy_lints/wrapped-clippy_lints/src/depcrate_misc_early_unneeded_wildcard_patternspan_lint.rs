// Generated macro for span_lint (function)
macro_rules! Depcrate_misc_early_unneeded_wildcard_patternspan_lint {
() => {
// Module: crate::misc_early::unneeded_wildcard_pattern
// Provides: {"span_lint"}
// Dependencies: {}
fn span_lint (cx : & EarlyContext < '_ > , span : Span , only_one : bool) { span_lint_and_sugg (cx , UNNEEDED_WILDCARD_PATTERN , span , if only_one { "this pattern is unneeded as the `..` pattern can match that element" } else { "these patterns are unneeded as the `..` pattern can match those elements" } , if only_one { "remove it" } else { "remove them" } , String :: new () , Applicability :: MachineApplicable ,) ; }
};
}
