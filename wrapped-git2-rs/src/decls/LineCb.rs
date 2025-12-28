macro_rules! deps {
    () => {
        DiffDelta!();
        DiffHunk!();
        DiffLine!();
    };
}

macro_rules! LineCb {
    () => {
        deps!();
        pub type LineCb < 'a > = dyn FnMut (DiffDelta < '_ > , Option < DiffHunk < '_ > > , DiffLine < '_ >) -> bool + 'a ;
    };
}

LineCb!();