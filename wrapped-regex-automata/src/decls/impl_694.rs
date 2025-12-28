macro_rules! deps {
    () => {
        LookMatcher!();
    };
}

macro_rules! impl_694 {
    () => {
        deps!();
        impl Default for LookMatcher { fn default () -> LookMatcher { LookMatcher :: new () } }
    };
}

impl_694!();