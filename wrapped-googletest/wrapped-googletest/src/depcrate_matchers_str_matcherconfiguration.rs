// Generated macro for Configuration (struct)
macro_rules! Depcrate_matchers_str_matcherConfiguration {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"Configuration"}
// Dependencies: {}
struct Configuration { mode : MatchMode , ignore_leading_whitespace : bool , ignore_trailing_whitespace : bool , case_policy : CasePolicy , times : Option < Box < dyn Matcher < usize > > > , }
};
}
