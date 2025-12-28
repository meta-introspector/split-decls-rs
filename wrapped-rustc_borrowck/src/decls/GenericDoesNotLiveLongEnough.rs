macro_rules! GenericDoesNotLiveLongEnough {
    () => {
        # [derive (Diagnostic)] # [diag (borrowck_generic_does_not_live_long_enough)] pub (crate) struct GenericDoesNotLiveLongEnough { pub kind : String , # [primary_span] pub span : Span , }
    };
}

GenericDoesNotLiveLongEnough!()