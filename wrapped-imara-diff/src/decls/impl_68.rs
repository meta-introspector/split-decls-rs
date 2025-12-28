macro_rules! deps {
    () => {
        Lines!();
        Token!();
        TokenSource!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [doc = " By default a line diff is produced for a string"] impl < 'a > TokenSource for Lines < 'a > { type Token = & 'a str ; type Tokenizer = Self ; fn tokenize (& self) -> Self :: Tokenizer { * self } fn estimate_tokens (& self) -> u32 { self . 0 . estimate_tokens () } }
    };
}

impl_68!()