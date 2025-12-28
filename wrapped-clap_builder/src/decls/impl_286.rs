macro_rules! deps {
    () => {
        PossibleValuesParser!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl PossibleValuesParser { # [doc = " Verify the value is from an enumerated set of [`PossibleValue`][crate::builder::PossibleValue]."] pub fn new (values : impl Into < PossibleValuesParser >) -> Self { values . into () } }
    };
}

impl_286!()