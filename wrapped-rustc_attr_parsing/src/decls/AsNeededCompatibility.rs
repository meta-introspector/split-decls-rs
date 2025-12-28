macro_rules! AsNeededCompatibility {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_as_needed_compatibility)] pub (crate) struct AsNeededCompatibility { # [primary_span] pub span : Span , }
    };
}

AsNeededCompatibility!();