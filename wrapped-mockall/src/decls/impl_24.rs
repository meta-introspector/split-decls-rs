macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < Range < usize > > for TimesRange { fn from (r : Range < usize >) -> TimesRange { assert ! (r . end > r . start , "Backwards range") ; TimesRange (r) } }
    };
}

impl_24!();