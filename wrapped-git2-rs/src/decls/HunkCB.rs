macro_rules! deps {
    () => {
        DiffHunk!();
    };
}

macro_rules! HunkCB {
    () => {
        deps!();
        type HunkCB < 'a > = dyn FnMut (Option < DiffHunk < '_ > >) -> bool + 'a ;
    };
}

HunkCB!()