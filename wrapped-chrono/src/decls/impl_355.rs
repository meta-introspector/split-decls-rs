macro_rules! deps {
    () => {
        NaiveDateDaysIterator!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl FusedIterator for NaiveDateDaysIterator { }
    };
}

impl_355!()