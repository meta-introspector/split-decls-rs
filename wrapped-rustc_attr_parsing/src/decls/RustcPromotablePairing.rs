macro_rules! RustcPromotablePairing {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_rustc_promotable_pairing , code = E0717)] pub (crate) struct RustcPromotablePairing { # [primary_span] pub span : Span , }
    };
}

RustcPromotablePairing!();