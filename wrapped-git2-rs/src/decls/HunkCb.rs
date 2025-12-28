macro_rules! deps {
    () => {
        DiffDelta!();
        DiffHunk!();
    };
}

macro_rules! HunkCb {
    () => {
        deps!();
        pub type HunkCb < 'a > = dyn FnMut (DiffDelta < '_ > , DiffHunk < '_ >) -> bool + 'a ;
    };
}

HunkCb!()