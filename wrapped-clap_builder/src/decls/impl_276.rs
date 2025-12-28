macro_rules! deps {
    () => {
        OsStringValueParser!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl Default for OsStringValueParser { fn default () -> Self { Self :: new () } }
    };
}

impl_276!()