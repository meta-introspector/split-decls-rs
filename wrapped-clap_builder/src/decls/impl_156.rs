macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl From < std :: ops :: Range < usize > > for ValueRange { fn from (range : std :: ops :: Range < usize >) -> Self { let start_inclusive = range . start ; let end_inclusive = range . end . saturating_sub (1) ; Self :: raw (start_inclusive , end_inclusive) } }
    };
}

impl_156!();