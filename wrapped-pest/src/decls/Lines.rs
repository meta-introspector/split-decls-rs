macro_rules! deps {
    () => {
        LinesSpan!();
    };
}

macro_rules! Lines {
    () => {
        deps!();
        # [doc = " Line iterator for Spans, created by [`Span::lines()`]."] # [doc = ""] # [doc = " Iterates all lines that are at least _partially_ covered by the span. Yielding a `&str` for each."] # [doc = ""] # [doc = " [`Span::lines()`]: struct.Span.html#method.lines"] pub struct Lines < 'i > { inner : LinesSpan < 'i > , }
    };
}

Lines!()