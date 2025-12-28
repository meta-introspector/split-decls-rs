macro_rules! ident {
    () => {
        # [doc = " Creates a new [`Ident`] which can be tokenized."] pub fn ident (s : & str) -> Ident { Ident :: new (s , Span :: call_site ()) }
    };
}

ident!()