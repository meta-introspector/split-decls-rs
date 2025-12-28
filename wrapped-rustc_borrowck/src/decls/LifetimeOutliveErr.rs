macro_rules! LifetimeOutliveErr {
    () => {
        # [derive (Diagnostic)] # [diag (borrowck_lifetime_constraints_error)] pub (crate) struct LifetimeOutliveErr { # [primary_span] pub span : Span , }
    };
}

LifetimeOutliveErr!()