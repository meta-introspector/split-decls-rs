macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl From < std :: ops :: RangeInclusive < usize > > for ValueRange { fn from (range : std :: ops :: RangeInclusive < usize >) -> Self { let start_inclusive = * range . start () ; let end_inclusive = * range . end () ; Self :: raw (start_inclusive , end_inclusive) } }
    };
}

impl_160!()