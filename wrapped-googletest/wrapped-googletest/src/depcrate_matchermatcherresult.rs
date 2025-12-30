// Generated macro for MatcherResult (enum)
macro_rules! Depcrate_matcherMatcherResult {
() => {
// Module: crate::matcher
// Provides: {"MatcherResult"}
// Dependencies: {}
# [doc = " The result of applying a [`Matcher`] on an actual value."] # [derive (Debug , PartialEq , Clone , Copy)] pub enum MatcherResult { # [doc = " The actual value matches according to the [`Matcher`] definition."] Match , # [doc = " The actual value does not match according to the [`Matcher`] definition."] NoMatch , }
};
}
