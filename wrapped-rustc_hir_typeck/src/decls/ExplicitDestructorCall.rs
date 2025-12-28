macro_rules! deps {
    () => {
        ExplicitDestructorCallSugg!();
    };
}

macro_rules! ExplicitDestructorCall {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_typeck_explicit_destructor , code = E0040)] pub (crate) struct ExplicitDestructorCall { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub sugg : ExplicitDestructorCallSugg , }
    };
}

ExplicitDestructorCall!();