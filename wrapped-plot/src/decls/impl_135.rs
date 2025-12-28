macro_rules! deps {
    () => {
        Figure!();
        Default!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl Default for Figure { fn default () -> Self { Self :: new () } }
    };
}

impl_135!();