macro_rules! GenericArgsOnOverriddenImpl {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_generic_args_on_overridden_impl)] pub (crate) struct GenericArgsOnOverriddenImpl { # [primary_span] pub span : Span , }
    };
}

GenericArgsOnOverriddenImpl!()