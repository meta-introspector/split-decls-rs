// Generated macro for create_assertion_failure (function)
macro_rules! Depcrate_matchercreate_assertion_failure {
() => {
// Module: crate::matcher
// Provides: {"create_assertion_failure"}
// Dependencies: {}
# [doc = " Constructs a [`TestAssertionFailure`] reporting that the given `matcher`"] # [doc = " does not match the value `actual`."] # [doc = ""] # [doc = " The parameter `actual_expr` contains the expression which was evaluated to"] # [doc = " obtain `actual`."] # [track_caller] pub (crate) fn create_assertion_failure < T : Debug + Copy > (matcher : & impl Matcher < T > , actual : T , actual_expr : & 'static str ,) -> TestAssertionFailure { let actual_formatted = format ! ("{actual:?}") ; let actual_formatted = if actual_formatted . len () > PRETTY_PRINT_LENGTH_THRESHOLD { format ! ("{actual:#?}") } else { actual_formatted } ; TestAssertionFailure :: create (format ! ("\
Value of: {actual_expr}
Expected: {}
Actual: {actual_formatted},
{}" , matcher . describe (MatcherResult :: Match) , matcher . explain_match (actual) . indent () ,)) }
};
}
