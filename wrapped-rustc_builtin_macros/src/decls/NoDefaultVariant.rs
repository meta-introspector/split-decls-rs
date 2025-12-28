macro_rules! deps {
    () => {
        NoDefaultVariantSugg!();
    };
}

macro_rules! NoDefaultVariant {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (builtin_macros_no_default_variant , code = E0665)] pub (crate) struct NoDefaultVariant { # [primary_span] pub (crate) span : Span , # [label] pub (crate) item_span : Span , # [subdiagnostic] pub (crate) suggs : Vec < NoDefaultVariantSugg > , }
    };
}

NoDefaultVariant!()