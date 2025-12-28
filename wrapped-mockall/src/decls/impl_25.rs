macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl From < RangeFrom < usize > > for TimesRange { fn from (r : RangeFrom < usize >) -> TimesRange { TimesRange (r . start .. usize :: MAX) } }
    };
}

impl_25!();