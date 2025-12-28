macro_rules! ConstBoundTraitObject {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_const_bound_trait_object)] pub (crate) struct ConstBoundTraitObject { # [primary_span] pub span : Span , }
    };
}

ConstBoundTraitObject!()