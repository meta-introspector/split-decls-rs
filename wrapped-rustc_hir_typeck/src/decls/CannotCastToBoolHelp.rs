macro_rules! CannotCastToBoolHelp {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum CannotCastToBoolHelp { # [suggestion (hir_typeck_suggestion , applicability = "machine-applicable" , code = " != 0" , style = "verbose")] Numeric (# [primary_span] Span) , # [label (hir_typeck_label)] Unsupported (# [primary_span] Span) , }
    };
}

CannotCastToBoolHelp!();