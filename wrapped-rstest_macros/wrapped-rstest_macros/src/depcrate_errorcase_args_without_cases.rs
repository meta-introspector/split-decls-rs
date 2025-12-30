// Generated macro for case_args_without_cases (function)
macro_rules! Depcrate_errorcase_args_without_cases {
() => {
// Module: crate::error
// Provides: {"case_args_without_cases"}
// Dependencies: {}
fn case_args_without_cases (params : & RsTestData) -> Errors < '_ > { if ! params . has_cases () { return Box :: new (params . case_args () . map (| a | syn :: Error :: new (a . span () , "No cases for this argument.")) ,) ; } Box :: new (std :: iter :: empty ()) }
};
}
