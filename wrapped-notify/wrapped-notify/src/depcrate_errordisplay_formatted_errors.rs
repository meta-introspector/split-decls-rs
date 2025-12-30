// Generated macro for display_formatted_errors (function)
macro_rules! Depcrate_errordisplay_formatted_errors {
() => {
// Module: crate::error
// Provides: {"display_formatted_errors"}
// Dependencies: {}
# [test] fn display_formatted_errors () { let expected = "Some error" ; assert_eq ! (expected , format ! ("{}" , Error :: generic (expected))) ; assert_eq ! (expected , format ! ("{}" , Error :: io (io :: Error :: other (expected)))) ; }
};
}
