macro_rules! deps {
    () => {
        IntervalSet!();
        Interval!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < I : Interval > Eq for IntervalSet < I > { }
    };
}

impl_145!()