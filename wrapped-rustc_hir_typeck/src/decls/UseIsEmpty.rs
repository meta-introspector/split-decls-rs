macro_rules! UseIsEmpty {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (hir_typeck_use_is_empty , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct UseIsEmpty < 'tcx > { # [suggestion_part (code = "!")] pub lo : Span , # [suggestion_part (code = ".is_empty()")] pub hi : Span , pub expr_ty : Ty < 'tcx > , }
    };
}

UseIsEmpty!()