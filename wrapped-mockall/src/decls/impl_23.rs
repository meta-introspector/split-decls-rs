macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < RangeTo < usize > > for TimesRange { fn from (r : RangeTo < usize >) -> TimesRange { TimesRange (0 .. r . end) } }
    };
}

impl_23!()