macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl From < std :: ops :: RangeTo < usize > > for ValueRange { fn from (range : std :: ops :: RangeTo < usize >) -> Self { let start_inclusive = 0 ; let end_inclusive = range . end . saturating_sub (1) ; Self :: raw (start_inclusive , end_inclusive) } }
    };
}

impl_159!()