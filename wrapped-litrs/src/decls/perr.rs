macro_rules! deps {
    () => {
        ParseErrorKind!();
        SpanLike!();
        ParseError!();
    };
}

macro_rules! perr {
    () => {
        deps!();
        # [doc = " This is a free standing function instead of an associated one to reduce"] # [doc = " noise around parsing code. There are lots of places that create errors, we"] # [doc = " I wanna keep them as short as possible."] pub (crate) fn perr (span : impl SpanLike , kind : ParseErrorKind) -> ParseError { ParseError { span : span . into_span () , kind , } }
    };
}

perr!()