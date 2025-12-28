macro_rules! deps {
    () => {
        SuggestAnnotations!();
    };
}

macro_rules! DependencyOnUnitNeverTypeFallback {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [help] # [diag (hir_typeck_dependency_on_unit_never_type_fallback)] pub (crate) struct DependencyOnUnitNeverTypeFallback < 'tcx > { # [note] pub obligation_span : Span , pub obligation : ty :: Predicate < 'tcx > , # [subdiagnostic] pub sugg : SuggestAnnotations , }
    };
}

DependencyOnUnitNeverTypeFallback!()