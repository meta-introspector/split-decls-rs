macro_rules! RTNSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (ast_lowering_bad_return_type_notation_output_suggestion , applicability = "machine-applicable" , style = "verbose")] # [doc = " Given `T: Tr<m() -> Ret>` or `T: Tr<m(Ty) -> Ret>`, suggest `T: Tr<m(..)>`."] pub (crate) struct RTNSuggestion { # [suggestion_part (code = "")] pub output : Span , # [suggestion_part (code = "(..)")] pub input : Span , }
    };
}

RTNSuggestion!()