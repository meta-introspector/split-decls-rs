macro_rules! NestedSpan {
    () => {
        # [derive (Clone , Copy)] struct NestedSpan { span : Span , nested_field_span : Span , }
    };
}

NestedSpan!();