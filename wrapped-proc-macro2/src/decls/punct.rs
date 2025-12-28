macro_rules! deps {
    () => {
        Punct!();
        Spacing!();
        Cursor!();
        PResult!();
        Reject!();
    };
}

macro_rules! punct {
    () => {
        deps!();
        fn punct (input : Cursor) -> PResult < Punct > { let (rest , ch) = punct_char (input) ? ; if ch == '\'' { let (after_lifetime , _ident) = ident_any (rest) ? ; if after_lifetime . starts_with_char ('\'') || (after_lifetime . starts_with_char ('#') && ! rest . starts_with ("r#")) { Err (Reject) } else { Ok ((rest , Punct :: new ('\'' , Spacing :: Joint))) } } else { let kind = match punct_char (rest) { Ok (_) => Spacing :: Joint , Err (Reject) => Spacing :: Alone , } ; Ok ((rest , Punct :: new (ch , kind))) } }
    };
}

punct!();