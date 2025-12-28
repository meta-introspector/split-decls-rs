macro_rules! SelfInTypeAlias {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_self_in_type_alias , code = E0411)] pub (crate) struct SelfInTypeAlias { # [primary_span] # [label] pub span : Span , }
    };
}

SelfInTypeAlias!()