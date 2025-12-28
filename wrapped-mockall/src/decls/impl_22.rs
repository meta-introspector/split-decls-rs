macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl From < RangeInclusive < usize > > for TimesRange { fn from (r : RangeInclusive < usize >) -> TimesRange { assert ! (r . end () >= r . start () , "Backwards range") ; TimesRange (* r . start () .. * r . end () + 1) } }
    };
}

impl_22!()