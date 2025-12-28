macro_rules! EmptyAttributeList {
    () => {
        # [derive (LintDiagnostic)] # [diag (attr_parsing_empty_attribute)] pub (crate) struct EmptyAttributeList { # [suggestion (code = "" , applicability = "machine-applicable")] pub attr_span : Span , }
    };
}

EmptyAttributeList!();