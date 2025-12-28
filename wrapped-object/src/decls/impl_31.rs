macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Default for BigEndian { # [inline] fn default () -> BigEndian { BigEndian } }
    };
}

impl_31!();