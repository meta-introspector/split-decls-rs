macro_rules! RawDylibNoNul {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_raw_dylib_no_nul)] pub (crate) struct RawDylibNoNul { # [primary_span] pub span : Span , }
    };
}

RawDylibNoNul!();