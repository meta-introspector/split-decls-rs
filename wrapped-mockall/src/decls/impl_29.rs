macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < RangeToInclusive < usize > > for TimesRange { fn from (r : RangeToInclusive < usize >) -> TimesRange { TimesRange (0 .. r . end + 1) } }
    };
}

impl_29!();