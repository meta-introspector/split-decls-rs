// Generated macro for check_fn_decl (function)
macro_rules! Depcrate_excessive_boolscheck_fn_decl {
() => {
// Module: crate::excessive_bools
// Provides: {"check_fn_decl"}
// Dependencies: {}
fn check_fn_decl (cx : & LateContext < '_ > , decl : & FnDecl < '_ > , sp : Span , max : u64) { if has_n_bools (decl . inputs . iter () , max) && ! sp . from_expansion () { span_lint_and_help (cx , FN_PARAMS_EXCESSIVE_BOOLS , sp , format ! ("more than {max} bools in function parameters") , None , "consider refactoring bools into two-variant enums" ,) ; } }
};
}
