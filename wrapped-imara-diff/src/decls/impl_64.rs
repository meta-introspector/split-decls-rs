macro_rules! deps {
    () => {
        Token!();
        TokenSource!();
        Lines!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        # [doc = " By default, a line diff is produced for a string"] impl < 'a > TokenSource for & 'a str { type Token = & 'a str ; type Tokenizer = Lines < 'a > ; fn tokenize (& self) -> Self :: Tokenizer { lines (self) } fn estimate_tokens (& self) -> u32 { lines (self) . estimate_tokens () } }
    };
}

impl_64!();