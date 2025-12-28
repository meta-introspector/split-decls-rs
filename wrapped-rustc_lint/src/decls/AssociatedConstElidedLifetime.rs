macro_rules! AssociatedConstElidedLifetime {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_associated_const_elided_lifetime)] pub (crate) struct AssociatedConstElidedLifetime { # [suggestion (style = "verbose" , code = "{code}" , applicability = "machine-applicable")] pub span : Span , pub code : & 'static str , pub elided : bool , # [note] pub lifetimes_in_scope : MultiSpan , }
    };
}

AssociatedConstElidedLifetime!()