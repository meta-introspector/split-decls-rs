macro_rules! EnvNotUnicode {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_env_not_unicode)] pub (crate) struct EnvNotUnicode { # [primary_span] pub (crate) span : Span , pub (crate) var : Symbol , }
    };
}

EnvNotUnicode!()