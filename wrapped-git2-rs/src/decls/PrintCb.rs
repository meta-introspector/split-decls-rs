macro_rules! deps {
    () => {
        DiffDelta!();
        DiffHunk!();
        DiffLine!();
    };
}

macro_rules! PrintCb {
    () => {
        deps!();
        type PrintCb < 'a > = dyn FnMut (DiffDelta < '_ > , Option < DiffHunk < '_ > > , DiffLine < '_ >) -> bool + 'a ;
    };
}

PrintCb!()