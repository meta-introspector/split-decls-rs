macro_rules! deps {
    () => {
        Condvar!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl Default for Condvar { fn default () -> Self { Self :: new () } }
    };
}

impl_285!();