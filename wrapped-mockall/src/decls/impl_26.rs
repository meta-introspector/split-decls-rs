macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl From < RangeFull > for TimesRange { fn from (_ : RangeFull) -> TimesRange { TimesRange (0 .. usize :: MAX) } }
    };
}

impl_26!()