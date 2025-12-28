macro_rules! deps {
    () => {
        DiffHunk!();
        DiffDelta!();
    };
}

macro_rules! HunkCb {
    () => {
        deps!();
        pub type HunkCb < 'a > = dyn FnMut (DiffDelta < '_ > , DiffHunk < '_ >) -> bool + 'a ;
    };
}

HunkCb!();