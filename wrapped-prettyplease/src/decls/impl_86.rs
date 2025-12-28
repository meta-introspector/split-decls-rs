macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl From < TokenTree > for Token { fn from (tt : TokenTree) -> Self { match tt { TokenTree :: Group (group) => Token :: Group (group . delimiter () , group . stream ()) , TokenTree :: Ident (ident) => Token :: Ident (ident) , TokenTree :: Punct (punct) => Token :: Punct (punct . as_char () , punct . spacing ()) , TokenTree :: Literal (literal) => Token :: Literal (literal) , } } }
    };
}

impl_86!();