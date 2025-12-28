macro_rules! deps {
    () => {
        WeekdaySetIter!();
    };
}

macro_rules! impl_718 {
    () => {
        deps!();
        impl FusedIterator for WeekdaySetIter { }
    };
}

impl_718!();