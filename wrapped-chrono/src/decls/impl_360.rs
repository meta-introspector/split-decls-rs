macro_rules! deps {
    () => {
        NaiveDateWeeksIterator!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl FusedIterator for NaiveDateWeeksIterator { }
    };
}

impl_360!();