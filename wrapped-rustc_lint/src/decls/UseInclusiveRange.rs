macro_rules! UseInclusiveRange {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UseInclusiveRange < 'a > { # [suggestion (lint_range_use_inclusive_range , code = "{start}..={literal}{suffix}" , applicability = "machine-applicable")] WithoutParen { # [primary_span] sugg : Span , start : String , literal : u128 , suffix : & 'a str , } , # [multipart_suggestion (lint_range_use_inclusive_range , applicability = "machine-applicable")] WithParen { # [suggestion_part (code = "=")] eq_sugg : Span , # [suggestion_part (code = "{literal}{suffix}")] lit_sugg : Span , literal : u128 , suffix : & 'a str , } , }
    };
}

UseInclusiveRange!();