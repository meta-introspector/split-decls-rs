macro_rules! deps {
    () => {
        TokenTree!();
        RcVecIntoIter!();
    };
}

macro_rules! TokenTreeIter {
    () => {
        deps!();
        pub (crate) type TokenTreeIter = RcVecIntoIter < TokenTree > ;
    };
}

TokenTreeIter!()