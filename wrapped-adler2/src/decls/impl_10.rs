macro_rules! deps {
    () => {
        Adler32!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for Adler32 { # [inline] fn default () -> Self { Adler32 { a : 1 , b : 0 } } }
    };
}

impl_10!();