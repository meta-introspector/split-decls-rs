macro_rules! BuiltinAnonymousParams {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_anonymous_params)] pub (crate) struct BuiltinAnonymousParams < 'a > { # [suggestion (code = "_: {ty_snip}")] pub suggestion : (Span , Applicability) , pub ty_snip : & 'a str , }
    };
}

BuiltinAnonymousParams!();