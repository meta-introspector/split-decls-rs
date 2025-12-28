macro_rules! InvalidAbiSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (ast_lowering_invalid_abi_suggestion , code = "\"{suggestion}\"" , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct InvalidAbiSuggestion { # [primary_span] pub span : Span , pub suggestion : String , }
    };
}

InvalidAbiSuggestion!()