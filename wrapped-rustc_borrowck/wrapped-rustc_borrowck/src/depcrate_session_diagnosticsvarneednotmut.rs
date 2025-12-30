// Generated macro for VarNeedNotMut (struct)
macro_rules! Depcrate_session_diagnosticsVarNeedNotMut {
() => {
// Module: crate::session_diagnostics
// Provides: {"VarNeedNotMut"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (borrowck_var_does_not_need_mut)] pub (crate) struct VarNeedNotMut { # [suggestion (style = "short" , applicability = "machine-applicable" , code = "")] pub span : Span , }
};
}
