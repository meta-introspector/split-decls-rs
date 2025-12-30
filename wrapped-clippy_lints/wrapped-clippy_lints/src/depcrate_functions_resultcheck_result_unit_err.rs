// Generated macro for check_result_unit_err (function)
macro_rules! Depcrate_functions_resultcheck_result_unit_err {
() => {
// Module: crate::functions::result
// Provides: {"check_result_unit_err"}
// Dependencies: {}
fn check_result_unit_err (cx : & LateContext < '_ > , err_ty : Ty < '_ > , fn_header_span : Span , msrv : Msrv) { if err_ty . is_unit () && (! is_no_std_crate (cx) || msrv . meets (cx , msrvs :: ERROR_IN_CORE)) { span_lint_and_help (cx , RESULT_UNIT_ERR , fn_header_span , "this returns a `Result<_, ()>`" , None , "use a custom `Error` type instead" ,) ; } }
};
}
