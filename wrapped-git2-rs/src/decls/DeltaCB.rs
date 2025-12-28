macro_rules! deps {
    () => {
        DiffDelta!();
    };
}

macro_rules! DeltaCB {
    () => {
        deps!();
        type DeltaCB < 'a > = dyn FnMut (Option < DiffDelta < '_ > >) -> bool + 'a ;
    };
}

DeltaCB!()