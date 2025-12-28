macro_rules! FormatRedundantArgsSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_suggestion , applicability = "machine-applicable")] pub (crate) struct FormatRedundantArgsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , }
    };
}

FormatRedundantArgsSugg!();