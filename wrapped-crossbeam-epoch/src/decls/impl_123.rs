macro_rules! deps {
    () => {
        Entry!();
        Atomic!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl Default for Entry { # [doc = " Returns the empty entry."] fn default () -> Self { Self { next : Atomic :: null () , } } }
    };
}

impl_123!();