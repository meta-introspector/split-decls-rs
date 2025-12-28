macro_rules! deps {
    () => {
        Span!();
        BorrowedOrArc!();
    };
}

macro_rules! SpanOrLiteral {
    () => {
        deps!();
        # [derive (Debug , Clone)] enum SpanOrLiteral < 'i > { Span (Span < 'i >) , Literal (BorrowedOrArc < 'i >) , }
    };
}

SpanOrLiteral!();