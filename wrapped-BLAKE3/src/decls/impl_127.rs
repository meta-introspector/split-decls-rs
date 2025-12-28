macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl digest :: Reset for Hasher { # [inline] fn reset (& mut self) { self . reset () ; } }
    };
}

impl_127!();