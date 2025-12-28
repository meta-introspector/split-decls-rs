macro_rules! deps {
    () => {
        ByteLines!();
        Token!();
        TokenSource!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        # [doc = " By default a line diff is produced for a string"] impl < 'a > TokenSource for ByteLines < 'a > { type Token = & 'a [u8] ; type Tokenizer = Self ; fn tokenize (& self) -> Self :: Tokenizer { * self } fn estimate_tokens (& self) -> u32 { let len : usize = self . take (20) . map (| line | line . len ()) . sum () ; if len == 0 { 100 } else { (self . 0 . len () * 20 / len) as u32 } } }
    };
}

impl_71!()