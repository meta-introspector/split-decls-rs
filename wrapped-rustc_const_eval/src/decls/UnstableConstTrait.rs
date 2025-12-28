macro_rules! UnstableConstTrait {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_unstable_const_trait)] pub (crate) struct UnstableConstTrait { # [primary_span] pub span : Span , pub def_path : String , }
    };
}

UnstableConstTrait!()