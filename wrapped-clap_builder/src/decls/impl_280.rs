macro_rules! deps {
    () => {
        PathBufValueParser!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl Default for PathBufValueParser { fn default () -> Self { Self :: new () } }
    };
}

impl_280!()