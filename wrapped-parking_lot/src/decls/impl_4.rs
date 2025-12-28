macro_rules! deps {
    () => {
        Condvar!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Default for Condvar { # [inline] fn default () -> Condvar { Condvar :: new () } }
    };
}

impl_4!()