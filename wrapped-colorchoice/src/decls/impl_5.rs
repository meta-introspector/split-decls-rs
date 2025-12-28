macro_rules! deps {
    () => {
        AtomicChoice!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Default for AtomicChoice { fn default () -> Self { Self :: new () } }
    };
}

impl_5!();