macro_rules! deps {
    () => {
        TokenTree!();
        Literal!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl From < Literal > for TokenTree { fn from (g : Literal) -> Self { TokenTree :: Literal (g) } }
    };
}

impl_41!()