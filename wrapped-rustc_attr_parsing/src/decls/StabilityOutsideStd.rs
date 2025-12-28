macro_rules! StabilityOutsideStd {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_stability_outside_std , code = E0734)] pub (crate) struct StabilityOutsideStd { # [primary_span] pub span : Span , }
    };
}

StabilityOutsideStd!()