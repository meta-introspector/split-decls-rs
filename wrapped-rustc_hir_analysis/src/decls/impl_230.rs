macro_rules! deps {
    () => {
        FieldDeclSpan!();
        NestedSpan!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl From < NestedSpan > for FieldDeclSpan { fn from (span : NestedSpan) -> Self { Self :: Nested (span) } }
    };
}

impl_230!();