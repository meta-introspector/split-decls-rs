macro_rules! ReplaceCommaWithSemicolon {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (hir_typeck_replace_comma_with_semicolon , applicability = "machine-applicable" , style = "verbose" , code = "; ")] pub (crate) struct ReplaceCommaWithSemicolon { # [primary_span] pub comma_span : Span , pub descr : & 'static str , }
    };
}

ReplaceCommaWithSemicolon!()