macro_rules! deps {
    () => {
        TokenKind!();
        Token!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Token { fn new (kind : TokenKind , len : u32) -> Token { Token { kind , len } } }
    };
}

impl_26!()