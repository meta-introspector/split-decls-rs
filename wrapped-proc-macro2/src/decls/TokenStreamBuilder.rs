macro_rules! deps {
    () => {
        TokenTree!();
        RcVecBuilder!();
    };
}

macro_rules! TokenStreamBuilder {
    () => {
        deps!();
        pub (crate) struct TokenStreamBuilder { inner : RcVecBuilder < TokenTree > , }
    };
}

TokenStreamBuilder!();