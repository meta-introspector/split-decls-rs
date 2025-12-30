// Generated macro for displays_as (function)
macro_rules! Depcrate_matchers_display_matcherdisplays_as {
() => {
// Module: crate::matchers::display_matcher
// Provides: {"displays_as"}
// Dependencies: {}
# [doc = " Matches the string representation of types that implement `Display`."] # [doc = ""] # [doc = " ```ignore"] # [doc = " let result: impl Display = ...;"] # [doc = " verify_that!(result, displays_as(eq(format!(\"{}\", result))))?;"] # [doc = " ```"] pub fn displays_as < InnerMatcher : for < 'a > Matcher < & 'a str > > (inner : InnerMatcher ,) -> DisplayMatcher < InnerMatcher > { DisplayMatcher { inner } }
};
}
