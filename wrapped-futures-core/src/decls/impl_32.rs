macro_rules! deps {
    () => {
        AtomicWaker!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Default for AtomicWaker { fn default () -> Self { Self :: new () } }
    };
}

impl_32!()