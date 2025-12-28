macro_rules! InvalidFormatStringLabel {
    () => {
        # [derive (Subdiagnostic)] # [label (builtin_macros_second_label)] pub (crate) struct InvalidFormatStringLabel { # [primary_span] pub (crate) span : Span , pub (crate) label : String , }
    };
}

InvalidFormatStringLabel!();