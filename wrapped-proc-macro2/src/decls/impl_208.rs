macro_rules! deps {
    () => {
        TokenTree!();
        Group!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl From < Group > for TokenTree { fn from (g : Group) -> Self { TokenTree :: Group (g) } }
    };
}

impl_208!();