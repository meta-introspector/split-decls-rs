macro_rules! deps {
    () => {
        Punct!();
        TokenTree!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl From < Punct > for TokenTree { fn from (g : Punct) -> Self { TokenTree :: Punct (g) } }
    };
}

impl_210!();