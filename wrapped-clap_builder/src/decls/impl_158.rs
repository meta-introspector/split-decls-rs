macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl From < std :: ops :: RangeFrom < usize > > for ValueRange { fn from (range : std :: ops :: RangeFrom < usize >) -> Self { let start_inclusive = range . start ; let end_inclusive = usize :: MAX ; Self :: raw (start_inclusive , end_inclusive) } }
    };
}

impl_158!();