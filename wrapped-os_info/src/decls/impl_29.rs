macro_rules! deps {
    () => {
        Info!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Default for Info { fn default () -> Self { Self :: unknown () } }
    };
}

impl_29!();