macro_rules! deps {
    () => {
        Punct!();
        TokenTree!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl From < Punct > for TokenTree { fn from (g : Punct) -> Self { TokenTree :: Punct (g) } }
    };
}

impl_40!()