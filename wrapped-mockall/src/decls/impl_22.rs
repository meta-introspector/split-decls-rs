macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Default for TimesRange { fn default () -> TimesRange { TimesRange (0 .. usize :: MAX) } }
    };
}

impl_22!()