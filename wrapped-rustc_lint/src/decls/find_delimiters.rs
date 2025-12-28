macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! find_delimiters {
    () => {
        deps!();
        # [doc = " Given the span of `some_macro!(args);`, gives the span of `(` and `)`,"] # [doc = " and the type of (opening) delimiter used."] fn find_delimiters (cx : & LateContext < '_ > , span : Span) -> Option < (Span , Span , char) > { let snippet = cx . sess () . source_map () . span_to_snippet (span) . ok () ? ; let (open , open_ch) = snippet . char_indices () . find (| & (_ , c) | "([{" . contains (c)) ? ; let close = snippet . rfind (| c | ")]}" . contains (c)) ? ; Some ((span . from_inner (InnerSpan { start : open , end : open + 1 }) , span . from_inner (InnerSpan { start : close , end : close + 1 }) , open_ch ,)) }
    };
}

find_delimiters!()