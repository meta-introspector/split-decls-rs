macro_rules! deps {
    () => {
        FieldDeclSpan!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl From < Span > for FieldDeclSpan { fn from (span : Span) -> Self { Self :: NotNested (span) } }
    };
}

impl_229!();