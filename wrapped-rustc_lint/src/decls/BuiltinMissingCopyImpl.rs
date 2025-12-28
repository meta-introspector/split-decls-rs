macro_rules! BuiltinMissingCopyImpl {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_missing_copy_impl)] pub (crate) struct BuiltinMissingCopyImpl ;
    };
}

BuiltinMissingCopyImpl!();