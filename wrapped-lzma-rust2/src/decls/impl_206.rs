macro_rules! deps {
    () => {
        LzmaOptions!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl Default for LzmaOptions { fn default () -> Self { Self :: with_preset (6) } }
    };
}

impl_206!()