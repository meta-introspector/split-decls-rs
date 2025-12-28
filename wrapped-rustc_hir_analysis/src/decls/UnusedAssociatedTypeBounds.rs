macro_rules! UnusedAssociatedTypeBounds {
    () => {
        # [derive (LintDiagnostic)] # [diag (hir_analysis_unused_associated_type_bounds)] # [note] pub (crate) struct UnusedAssociatedTypeBounds { # [suggestion (code = "")] pub span : Span , }
    };
}

UnusedAssociatedTypeBounds!()