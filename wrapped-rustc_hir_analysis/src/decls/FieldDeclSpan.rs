macro_rules! deps {
    () => {
        NestedSpan!();
    };
}

macro_rules! FieldDeclSpan {
    () => {
        deps!();
        # [derive (Clone , Copy)] enum FieldDeclSpan { NotNested (Span) , Nested (NestedSpan) , }
    };
}

FieldDeclSpan!();