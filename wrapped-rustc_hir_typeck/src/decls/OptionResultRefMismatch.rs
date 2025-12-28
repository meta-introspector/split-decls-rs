macro_rules! OptionResultRefMismatch {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum OptionResultRefMismatch { # [suggestion (hir_typeck_option_result_copied , code = ".copied()" , style = "verbose" , applicability = "machine-applicable")] Copied { # [primary_span] span : Span , def_path : String , } , # [suggestion (hir_typeck_option_result_cloned , code = ".cloned()" , style = "verbose" , applicability = "machine-applicable")] Cloned { # [primary_span] span : Span , def_path : String , } , }
    };
}

OptionResultRefMismatch!()