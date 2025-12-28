macro_rules! deps {
    () => {
        RcVecBuilder!();
        TokenTree!();
    };
}

macro_rules! TokenStreamBuilder {
    () => {
        deps!();
        pub (crate) struct TokenStreamBuilder { inner : RcVecBuilder < TokenTree > , }
    };
}

TokenStreamBuilder!()