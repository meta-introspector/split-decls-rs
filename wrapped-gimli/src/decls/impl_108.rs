macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl Default for BigEndian { # [inline] fn default () -> BigEndian { BigEndian } }
    };
}

impl_108!();