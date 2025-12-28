macro_rules! deps {
    () => {
        TreeEntry!();
    };
}

macro_rules! TreeWalkCb {
    () => {
        deps!();
        type TreeWalkCb < 'a , T > = dyn FnMut (& str , & TreeEntry < '_ >) -> T + 'a ;
    };
}

TreeWalkCb!();