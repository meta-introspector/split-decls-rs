macro_rules! deps {
    () => {
        AtomicChoice!();
    };
}

macro_rules! USER {
    () => {
        deps!();
        static USER : AtomicChoice = AtomicChoice :: new () ;
    };
}

USER!();