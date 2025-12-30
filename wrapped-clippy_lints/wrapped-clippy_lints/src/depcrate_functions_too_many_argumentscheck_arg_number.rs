// Generated macro for check_arg_number (function)
macro_rules! Depcrate_functions_too_many_argumentscheck_arg_number {
() => {
// Module: crate::functions::too_many_arguments
// Provides: {"check_arg_number"}
// Dependencies: {}
fn check_arg_number (cx : & LateContext < '_ > , decl : & hir :: FnDecl < '_ > , fn_span : Span , too_many_arguments_threshold : u64) { let args = decl . inputs . len () as u64 ; if args > too_many_arguments_threshold { span_lint (cx , TOO_MANY_ARGUMENTS , fn_span , format ! ("this function has too many arguments ({args}/{too_many_arguments_threshold})") ,) ; } }
};
}
