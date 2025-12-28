macro_rules! InvalidMetaItemRemoveNegSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (attr_parsing_remove_neg_sugg , applicability = "machine-applicable")] pub (crate) struct InvalidMetaItemRemoveNegSugg { # [suggestion_part (code = "")] pub negative_sign : Span , }
    };
}

InvalidMetaItemRemoveNegSugg!()