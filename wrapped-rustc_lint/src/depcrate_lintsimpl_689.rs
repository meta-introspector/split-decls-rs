// Generated macro for impl_689 (impl)
macro_rules! Depcrate_lintsimpl_689 {
() => {
// Module: crate::lints
// Provides: {"impl_689"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for BuiltinMissingDebugImpl < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut rustc_errors :: Diag < 'a , () >) { diag . primary_message (fluent :: lint_builtin_missing_debug_impl) ; diag . arg ("debug" , self . tcx . def_path_str (self . def_id)) ; } }
};
}
