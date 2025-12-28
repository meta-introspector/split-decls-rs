macro_rules! deps {
    () => {
        ThreadRng!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl Default for ThreadRng { fn default () -> ThreadRng { rng () } }
    };
}

impl_256!();