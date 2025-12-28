macro_rules! BuiltinMissingDoc {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_missing_doc)] pub (crate) struct BuiltinMissingDoc < 'a > { pub article : & 'a str , pub desc : & 'a str , }
    };
}

BuiltinMissingDoc!();