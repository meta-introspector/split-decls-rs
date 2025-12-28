macro_rules! RawDylibOnlyWindows {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_raw_dylib_only_windows , code = E0455)] pub (crate) struct RawDylibOnlyWindows { # [primary_span] pub span : Span , }
    };
}

RawDylibOnlyWindows!();