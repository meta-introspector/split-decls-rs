macro_rules! deps {
    () => {
        Span!();
        BorrowedOrArc!();
        SpanOrLiteral!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 'i > SpanOrLiteral < 'i > { # [inline] fn as_borrowed_or_rc (& self) -> BorrowedOrArc < 'i > { match self { Self :: Span (s) => BorrowedOrArc :: Borrowed (s . as_str ()) , Self :: Literal (s) => s . clone () , } } }
    };
}

impl_105!()