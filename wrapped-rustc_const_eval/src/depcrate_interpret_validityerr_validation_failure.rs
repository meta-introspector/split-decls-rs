// Generated macro for err_validation_failure (macro)
macro_rules! Depcrate_interpret_validityerr_validation_failure {
() => {
// Module: crate::interpret::validity
// Provides: {"err_validation_failure"}
// Dependencies: {}
macro_rules ! err_validation_failure { ($ where : expr , $ kind : expr) => { { let where_ = &$ where ; let path = if ! where_ . is_empty () { let mut path = String :: new () ; write_path (& mut path , where_) ; Some (path) } else { None } ; err_ub ! (ValidationError (ValidationErrorInfo { path , kind : $ kind })) } } ; }
};
}
