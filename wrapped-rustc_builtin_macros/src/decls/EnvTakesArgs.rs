macro_rules! EnvTakesArgs {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_env_takes_args)] pub (crate) struct EnvTakesArgs { # [primary_span] pub (crate) span : Span , }
    };
}

EnvTakesArgs!();