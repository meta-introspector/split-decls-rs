macro_rules! deps {
    () => {
        Token!();
        TokenKind!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Token { fn new (kind : TokenKind , len : u32) -> Token { Token { kind , len } } }
    };
}

impl_3!()