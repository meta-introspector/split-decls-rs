macro_rules! deps {
    () => {
        Group!();
        TokenTree!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl From < Group > for TokenTree { fn from (g : Group) -> Self { TokenTree :: Group (g) } }
    };
}

impl_38!()