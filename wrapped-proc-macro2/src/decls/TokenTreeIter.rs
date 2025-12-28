macro_rules! deps {
    () => {
        RcVecIntoIter!();
        TokenTree!();
    };
}

macro_rules! TokenTreeIter {
    () => {
        deps!();
        pub (crate) type TokenTreeIter = RcVecIntoIter < TokenTree > ;
    };
}

TokenTreeIter!();