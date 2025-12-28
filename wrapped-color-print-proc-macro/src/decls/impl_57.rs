macro_rules! deps {
    () => {
        ExtColor!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Default for ExtColor { fn default () -> Self { Self :: Normal } }
    };
}

impl_57!();