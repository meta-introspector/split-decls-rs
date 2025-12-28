macro_rules! deps {
    () => {
        AtomicTargetSize!();
    };
}

macro_rules! Cell {
    () => {
        deps!();
        struct Cell < T > { data : MaybeUninit < T > , sequence : AtomicTargetSize , }
    };
}

Cell!()