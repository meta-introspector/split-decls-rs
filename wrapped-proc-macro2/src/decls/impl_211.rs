macro_rules! deps {
    () => {
        Literal!();
        TokenTree!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl From < Literal > for TokenTree { fn from (g : Literal) -> Self { TokenTree :: Literal (g) } }
    };
}

impl_211!();