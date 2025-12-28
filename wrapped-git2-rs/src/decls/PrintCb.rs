macro_rules! deps {
    () => {
        DiffDelta!();
        DiffLine!();
        DiffHunk!();
    };
}

macro_rules! PrintCb {
    () => {
        deps!();
        type PrintCb < 'a > = dyn FnMut (DiffDelta < '_ > , Option < DiffHunk < '_ > > , DiffLine < '_ >) -> bool + 'a ;
    };
}

PrintCb!();