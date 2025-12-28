macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < usize > for TimesRange { fn from (n : usize) -> TimesRange { TimesRange (n .. (n + 1)) } }
    };
}

impl_23!();