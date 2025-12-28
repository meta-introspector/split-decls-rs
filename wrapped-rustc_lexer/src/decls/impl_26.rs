macro_rules! deps {
    () => {
        Token!();
        TokenKind!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Token { fn new (kind : TokenKind , len : u32) -> Token { Token { kind , len } } }
    };
}

impl_26!();