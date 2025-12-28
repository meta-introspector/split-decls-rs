macro_rules! EmptyConfusables {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_empty_confusables)] pub (crate) struct EmptyConfusables { # [primary_span] pub span : Span , }
    };
}

EmptyConfusables!();