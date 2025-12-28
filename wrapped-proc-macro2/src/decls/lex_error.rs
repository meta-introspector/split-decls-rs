macro_rules! deps {
    () => {
        Span!();
        Cursor!();
        LexError!();
    };
}

macro_rules! lex_error {
    () => {
        deps!();
        fn lex_error (cursor : Cursor) -> LexError { # [cfg (not (span_locations))] let _ = cursor ; LexError { span : Span { # [cfg (span_locations)] lo : cursor . off , # [cfg (span_locations)] hi : cursor . off , } , } }
    };
}

lex_error!()