macro_rules! deps {
    () => {
        Token!();
        TokenKind!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl PartialEq < TokenKind > for Token { # [inline] fn eq (& self , rhs : & TokenKind) -> bool { self . kind == * rhs } }
    };
}

impl_412!()