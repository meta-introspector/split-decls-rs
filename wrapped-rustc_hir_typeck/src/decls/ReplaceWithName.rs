macro_rules! ReplaceWithName {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (hir_typeck_suggestion , code = "{name}" , applicability = "machine-applicable")] pub (crate) struct ReplaceWithName { # [primary_span] pub span : Span , pub name : String , }
    };
}

ReplaceWithName!()