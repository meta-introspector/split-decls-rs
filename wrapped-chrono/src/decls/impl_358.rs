macro_rules! deps {
    () => {
        NaiveDateWeeksIterator!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl ExactSizeIterator for NaiveDateWeeksIterator { }
    };
}

impl_358!()