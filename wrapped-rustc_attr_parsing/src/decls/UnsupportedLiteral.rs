macro_rules! deps {
    () => {
        UnsupportedLiteralReason!();
    };
}

macro_rules! UnsupportedLiteral {
    () => {
        deps!();
        # [doc = " Error code: E0565"] pub (crate) struct UnsupportedLiteral { pub span : Span , pub reason : UnsupportedLiteralReason , pub is_bytestr : bool , pub start_point_span : Span , }
    };
}

UnsupportedLiteral!();