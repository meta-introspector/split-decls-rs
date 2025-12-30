// Generated macro for impl_809 (impl)
macro_rules! Depcrate_lintsimpl_809 {
() => {
// Module: crate::lints
// Provides: {"impl_809"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for DropGlue < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_drop_glue) ; diag . arg ("needs_drop" , self . tcx . def_path_str (self . def_id)) ; } }
};
}
