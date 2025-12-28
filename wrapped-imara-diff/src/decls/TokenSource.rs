macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! TokenSource {
    () => {
        deps!();
        pub trait TokenSource { type Token : Hash + Eq ; type Tokenizer : Iterator < Item = Self :: Token > ; fn tokenize (& self) -> Self :: Tokenizer ; fn estimate_tokens (& self) -> u32 ; }
    };
}

TokenSource!();