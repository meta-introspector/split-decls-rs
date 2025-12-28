macro_rules! OnlyOneArgument {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_only_one_argument)] pub (crate) struct OnlyOneArgument < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , }
    };
}

OnlyOneArgument!()