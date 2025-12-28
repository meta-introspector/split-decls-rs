macro_rules! UnstableConstFn {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_unstable_const_fn)] pub (crate) struct UnstableConstFn { # [primary_span] pub span : Span , pub def_path : String , }
    };
}

UnstableConstFn!()