macro_rules! deps {
    () => {
        PositiveMeasure!();
    };
}

macro_rules! impl_positive_measure {
    () => {
        deps!();
        macro_rules ! impl_positive_measure (($ ($ t : ident) ,*) => { $ (impl PositiveMeasure for $ t { fn zero () -> Self { 0 as $ t } fn max () -> Self { $ t :: MAX } }) * }) ;
    };
}

impl_positive_measure!()