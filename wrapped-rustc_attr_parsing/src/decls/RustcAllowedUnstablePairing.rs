macro_rules! RustcAllowedUnstablePairing {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_rustc_allowed_unstable_pairing , code = E0789)] pub (crate) struct RustcAllowedUnstablePairing { # [primary_span] pub span : Span , }
    };
}

RustcAllowedUnstablePairing!();