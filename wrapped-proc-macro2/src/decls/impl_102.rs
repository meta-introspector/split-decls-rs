macro_rules! deps {
    () => {
        TokenTreeIter!();
        TokenTree!();
        TokenStream!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl IntoIterator for TokenStream { type Item = TokenTree ; type IntoIter = TokenTreeIter ; fn into_iter (self) -> TokenTreeIter { self . take_inner () . into_iter () } }
    };
}

impl_102!()