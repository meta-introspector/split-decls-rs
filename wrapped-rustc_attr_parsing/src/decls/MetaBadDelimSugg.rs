macro_rules! MetaBadDelimSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (attr_parsing_meta_bad_delim_suggestion , applicability = "machine-applicable")] pub (crate) struct MetaBadDelimSugg { # [suggestion_part (code = "(")] pub open : Span , # [suggestion_part (code = ")")] pub close : Span , }
    };
}

MetaBadDelimSugg!();