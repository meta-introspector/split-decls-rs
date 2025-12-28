macro_rules! InvalidFormatStringSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum InvalidFormatStringSuggestion { # [multipart_suggestion (builtin_macros_format_use_positional , style = "verbose" , applicability = "machine-applicable")] UsePositional { # [suggestion_part (code = "{len}")] captured : Span , len : String , # [suggestion_part (code = ", {arg}")] span : Span , arg : String , } , # [suggestion (builtin_macros_format_remove_raw_ident , code = "" , applicability = "machine-applicable")] RemoveRawIdent { # [primary_span] span : Span , } , # [suggestion (builtin_macros_format_reorder_format_parameter , code = "{replacement}" , style = "verbose" , applicability = "machine-applicable")] ReorderFormatParameter { # [primary_span] span : Span , replacement : String , } , }
    };
}

InvalidFormatStringSuggestion!();