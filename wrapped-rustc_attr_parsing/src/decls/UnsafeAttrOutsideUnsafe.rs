macro_rules! deps {
    () => {
        UnsafeAttrOutsideUnsafeSuggestion!();
    };
}

macro_rules! UnsafeAttrOutsideUnsafe {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (attr_parsing_unsafe_attr_outside_unsafe)] pub (crate) struct UnsafeAttrOutsideUnsafe { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub suggestion : UnsafeAttrOutsideUnsafeSuggestion , }
    };
}

UnsafeAttrOutsideUnsafe!();