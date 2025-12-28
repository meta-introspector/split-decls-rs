macro_rules! literal_string {
    () => {
        # [doc = " Creates a new [`struct@LitStr`] which can be tokenized."] pub fn literal_string (s : & str) -> LitStr { LitStr :: new (s , Span :: call_site ()) }
    };
}

literal_string!();