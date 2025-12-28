macro_rules! deps {
    () => {
        Measure!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl < M > Measure for M where M : Debug + PartialOrd + Add < M , Output = M > + Default + Clone { }
    };
}

impl_493!()