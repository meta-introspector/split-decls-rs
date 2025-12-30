// Generated macro for show_error (function)
macro_rules! Depcrate_non_octal_unix_permissionsshow_error {
() => {
// Module: crate::non_octal_unix_permissions
// Provides: {"show_error"}
// Dependencies: {}
fn show_error (cx : & LateContext < '_ > , param : & Expr < '_ >) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , NON_OCTAL_UNIX_PERMISSIONS , param . span , "using a non-octal value to set unix file permissions" , "consider using an octal literal instead" , format ! ("0o{}" , snippet_with_applicability (cx , param . span , "0o.." , & mut applicability ,) ,) , applicability ,) ; }
};
}
