macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl From < std :: ops :: RangeToInclusive < usize > > for ValueRange { fn from (range : std :: ops :: RangeToInclusive < usize >) -> Self { let start_inclusive = 0 ; let end_inclusive = range . end ; Self :: raw (start_inclusive , end_inclusive) } }
    };
}

impl_161!()