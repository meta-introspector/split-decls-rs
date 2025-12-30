// Generated macro for UselessPtrNullChecksDiag (enum)
macro_rules! Depcrate_lintsUselessPtrNullChecksDiag {
() => {
// Module: crate::lints
// Provides: {"UselessPtrNullChecksDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum UselessPtrNullChecksDiag < 'a > { # [diag (lint_useless_ptr_null_checks_fn_ptr)] # [help] FnPtr { orig_ty : Ty < 'a > , # [label] label : Span , } , # [diag (lint_useless_ptr_null_checks_ref)] Ref { orig_ty : Ty < 'a > , # [label] label : Span , } , # [diag (lint_useless_ptr_null_checks_fn_ret)] FnRet { fn_name : Ident } , }
};
}
