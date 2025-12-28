macro_rules! deps {
    () => {
        TokenTree!();
        Ident!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl From < Ident > for TokenTree { fn from (g : Ident) -> Self { TokenTree :: Ident (g) } }
    };
}

impl_209!()