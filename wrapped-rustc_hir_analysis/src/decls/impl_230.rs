macro_rules! deps {
    () => {
        NestedSpan!();
        FieldDeclSpan!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl From < NestedSpan > for FieldDeclSpan { fn from (span : NestedSpan) -> Self { Self :: Nested (span) } }
    };
}

impl_230!()