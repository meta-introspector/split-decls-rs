macro_rules! deps {
    () => {
        NaiveDateDaysIterator!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl ExactSizeIterator for NaiveDateDaysIterator { }
    };
}

impl_353!();